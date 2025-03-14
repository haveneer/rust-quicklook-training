use crate::ParseError;
use std::convert::TryInto;

/// An iterator over a binary stream of blocks.
///
/// This iterator goes through a byte slice and successively returns each parsed `Block`.
/// In case of a format error or incomplete data, it returns a `ParseError`.
pub struct BlockViewIterator<'a> {
    data: &'a [u8],
    offset: usize,
    finished: bool,
}

impl<'a> BlockViewIterator<'a> {
    /// Creates a new block iterator from the raw `data`.
    pub fn new(data: &'a [u8]) -> Self {
        BlockViewIterator {
            data,
            offset: 0,
            finished: false,
        }
    }
}

impl<'a> Iterator for BlockViewIterator<'a> {
    type Item = Result<BlockView<'a>, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        // If we have already reached the end or encountered a blocking error, stop iteration.
        if self.finished || self.offset >= self.data.len() {
            return None;
        }

        // Attempt to parse the next block from self.offset.
        let start = self.offset;
        let data = self.data;

        // Need at least 16 bytes for the block header (id, timestamp, transaction count).
        if data.len() < start + 16 {
            self.finished = true; // stop iteration
            return Some(Err(ParseError::UnexpectedEOF));
        }

        // Read the number of transactions (4 bytes little-endian)
        let tx_count_bytes: [u8; 4] = data[start + 12..start + 16].try_into().unwrap();
        let tx_count = u32::from_le_bytes(tx_count_bytes) as usize;

        // Compute the start position of transactions and begin traversal.
        let mut pos = start + 16;
        // Loop to skip all transactions based on their lengths.
        for _ in 0..tx_count {
            // Check availability of transaction header (4-byte length)
            if pos + 4 > data.len() {
                self.finished = true;
                return Some(Err(ParseError::UnexpectedEOF));
            }
            let len_bytes: [u8; 4] = data[pos..pos + 4].try_into().unwrap();
            let tx_len = u32::from_le_bytes(len_bytes) as usize;
            pos += 4;
            // Ensure that transaction data is fully present.
            if pos + tx_len > data.len() {
                self.finished = true;
                return Some(Err(ParseError::UnexpectedEOF));
            }
            pos += tx_len;
        }

        // After the loop, pos is where the previous block's hash should start.
        // Check for the presence of a 32-byte hash.
        if pos + 32 > data.len() {
            self.finished = true;
            return Some(Err(ParseError::UnexpectedEOF));
        }
        let block_end = pos + 32;
        // Slice the data corresponding exactly to this block.
        let block_slice = &data[start..block_end];
        // Update the offset for the next block.
        self.offset = block_end;
        // If we have reached the end of the data, mark as finished to stop iterating.
        if self.offset >= data.len() {
            self.finished = true;
        }

        // Construct the Block with the current block's slice.
        Some(Ok(BlockView { data: block_slice }))
    }
}

/// Represents a parsed block containing a reference to the raw block data.
///
/// The block's fields (`id`, `timestamp`, etc.) are extracted on demand from the raw data.
/// This structure does not copy the block's data (zero-copy).
#[derive(Debug)]
pub struct BlockView<'a> {
    data: &'a [u8],
}

impl<'a> BlockView<'a> {
    /// Returns the block identifier (`id` field, 4 bytes).
    pub fn id(&self) -> u32 {
        let bytes: [u8; 4] = self.data[0..4].try_into().unwrap();
        u32::from_le_bytes(bytes)
    }

    /// Returns the block timestamp (`timestamp` field, 8 bytes).
    pub fn timestamp(&self) -> u64 {
        let bytes: [u8; 8] = self.data[4..12].try_into().unwrap();
        u64::from_le_bytes(bytes)
    }

    /// Returns the number of transactions contained in the block.
    pub fn transaction_count(&self) -> u32 {
        let bytes: [u8; 4] = self.data[12..16].try_into().unwrap();
        u32::from_le_bytes(bytes)
    }

    /// Returns the previous block hash (32 bytes) as a slice.
    pub fn prev_hash(&self) -> &'a [u8] {
        // The hash is located at the end of the block: last 32 bytes.
        let start_hash = self.data.len() - 32;
        &self.data[start_hash..self.data.len()]
    }

    /// Provides an iterator over the block's transactions.
    ///
    /// Each produced element is a transaction represented by a `Transaction` structure
    /// containing a slice of bytes (the original transaction data).
    pub fn transactions(&self) -> TransactionViewIterator<'a> {
        // The transaction section starts at byte 16 and ends just before the 32-byte hash.
        let tx_count = self.transaction_count();
        let tx_data_start = 16;
        let tx_data_end = self.data.len() - 32;
        // Sub-slice containing only the transaction area (all concatenated).
        let tx_data_slice = &self.data[tx_data_start..tx_data_end];
        TransactionViewIterator {
            data: tx_data_slice,
            tx_count,
            current_index: 0,
            offset: 0,
        }
    }

    pub fn transactions_bytes_sum(&self) -> u64 {
        self.transactions().fold(0, |acc, tx| {
            acc + tx.data().iter().map(|&b| b as u64).sum::<u64>()
        })
    }
}

/// Represents a transaction extracted from a block, as a data slice.
///
/// This structure is zero-copy: it only references the transaction bytes.
pub struct TransactionView<'a> {
    data: &'a [u8],
}

impl<'a> TransactionView<'a> {
    /// Returns the raw transaction data as a slice.
    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    /// Returns the transaction's size (length in bytes).
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Iterator over the transactions of a block.
///
/// This iterator sequentially goes through each transaction in a slice of transactions.
pub struct TransactionViewIterator<'a> {
    data: &'a [u8],
    tx_count: u32,
    current_index: u32,
    offset: usize,
}

impl<'a> Iterator for TransactionViewIterator<'a> {
    type Item = TransactionView<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.tx_count {
            return None;
        }
        // Read the current transaction length (4-byte LE).
        let len_bytes: [u8; 4] = self.data[self.offset..self.offset + 4].try_into().unwrap();
        let tx_len = u32::from_le_bytes(len_bytes) as usize;
        self.offset += 4;
        // Slice corresponding to the transaction data.
        let start = self.offset;
        let end = self.offset + tx_len;
        let tx_slice = &self.data[start..end];
        // Update the iterator state.
        self.offset = end;
        self.current_index += 1;
        Some(TransactionView { data: tx_slice })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_interfaces::Transactions;
    use crate::{make_test_block, Block, ToBytes, Transaction};

    #[test]
    fn test_single_block_parsing() {
        let ref_block = make_test_block();
        let data = ref_block.to_bytes();

        // Parsing du bloc unique.
        let mut iter = BlockViewIterator::new(&data);
        let block = iter
            .next()
            .expect("Un bloc attendu")
            .expect("Pas d'erreur attendue");
        // Vérifications des champs du bloc.
        assert_eq!(block.id(), ref_block.id());
        assert_eq!(block.timestamp(), ref_block.timestamp());
        assert_eq!(
            block.transaction_count(),
            ref_block.transactions().len() as u32
        );

        for (ref_tx, tx) in std::iter::zip(block.transactions(), ref_block.transactions().iter()) {
            assert_eq!(ref_tx.data(), tx.data());
        }
        assert_eq!(block.prev_hash(), ref_block.prev_hash());
    }

    #[test]
    fn test_multiple_blocks_iteration() {
        let block1 = make_test_block();
        let block2 = make_test_block();

        let mut data = Vec::new();
        data.extend(block1.to_bytes().as_slice());
        data.extend(block2.to_bytes().as_slice());

        // Itération sur le flux de deux blocs.
        let mut iter = BlockViewIterator::new(&data);
        // Premier bloc
        let b1 = iter
            .next()
            .expect("Bloc 1 présent")
            .expect("Bloc 1 sans erreur");
        assert_eq!(b1.id(), block1.id());
        assert_eq!(b1.timestamp(), block1.timestamp());
        assert_eq!(b1.transaction_count(), block1.transactions().len() as u32);
        assert_eq!(b1.transactions().count(), block1.transactions().len());
        let txs1: Vec<_> = b1.transactions().collect();
        for (ref_tx, tx) in std::iter::zip(block1.transactions().iter(), txs1.iter()) {
            assert_eq!(ref_tx.data(), tx.data());
        }
        assert_eq!(b1.prev_hash(), block1.prev_hash());

        // Deuxième bloc
        let b2 = iter
            .next()
            .expect("Bloc 2 présent")
            .expect("Bloc 2 sans erreur");
        assert_eq!(b2.id(), block1.id());
        assert_eq!(b2.timestamp(), block1.timestamp());
        assert_eq!(b2.transaction_count(), block1.transactions().len() as u32);
        assert_eq!(b2.transactions().count(), block1.transactions().len());
        let txs2: Vec<TransactionView> = b2.transactions().collect();
        for (ref_tx, tx) in std::iter::zip(block1.transactions().iter(), txs2.iter()) {
            assert_eq!(ref_tx.data(), tx.data());
        }
        assert_eq!(b2.prev_hash(), block1.prev_hash());

        // Il ne doit plus y avoir de bloc après.
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_error_incomplete_data() {
        let data = make_test_block().to_bytes();
        // exclude full size data
        for cut in 1..data.len() {
            let data = &data[0..cut];
            let mut iter = BlockViewIterator::new(&data);
            let result = iter.next();
            assert!(result.is_some());
            let err = result.unwrap().unwrap_err();
            assert_eq!(err, ParseError::UnexpectedEOF);
            // Pas d'autre bloc après, l'itérateur est fini.
            assert!(iter.next().is_none());
        }
    }
}
