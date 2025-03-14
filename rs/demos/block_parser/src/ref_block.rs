use crate::block_interfaces::{Block, FromBytes, Transaction, Transactions};
use crate::ParseError;
use std::convert::TryInto;

#[derive(Debug, PartialEq, Eq)]
pub struct RefBlock<'a> {
    data: &'a [u8],
}

impl Block for RefBlock<'_> {
    fn id(&self) -> u32 {
        u32::from_le_bytes(self.data[0..4].try_into().unwrap())
    }
    fn timestamp(&self) -> u64 {
        u64::from_le_bytes(self.data[4..12].try_into().unwrap())
    }
    fn transactions(&self) -> impl Transactions {
        let len = u32::from_le_bytes(self.data[12..16].try_into().unwrap());
        RefTransactions {
            len: len as usize,
            index: 0,
            offset: 0,
            data: &self.data[16..self.data.len() - 32],
        }
    }
    fn prev_hash(&self) -> &[u8; 32] {
        let len = self.data.len();
        self.data[len - 32..len].try_into().unwrap()
    }
}

pub struct RefTransactions<'a> {
    len: usize,
    index: usize,
    offset: usize,
    data: &'a [u8],
}

impl Transactions for RefTransactions<'_> {
    type Transaction<'b>
        = RefTransaction<'b>
    where
        Self: 'b;

    fn len(&self) -> usize {
        self.len
    }

    fn iter(&self) -> impl Iterator<Item = Self::Transaction<'_>> {
        RefTransactionsIterator {
            len: self.len,
            index: self.index,
            offset: self.offset,
            data: self.data,
        }
    }
}

pub struct RefTransaction<'a> {
    data: &'a [u8],
}

impl Transaction for RefTransaction<'_> {
    fn data(&self) -> &[u8] {
        self.data
    }
}

pub struct RefTransactionsIterator<'a> {
    len: usize,
    index: usize,
    offset: usize,
    data: &'a [u8],
}

impl<'a> Iterator for RefTransactionsIterator<'a> {
    type Item = RefTransaction<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            self.index += 1;
            let len =
                u32::from_le_bytes(self.data[self.offset..self.offset + 4].try_into().unwrap())
                    as usize;
            let offset = self.offset + 4;
            self.offset += 4 + len;
            Some(RefTransaction {
                data: &self.data[offset..offset + len],
            })
        } else {
            None
        }
    }
}

impl<'a> FromBytes<'a> for RefBlock<'a> {
    fn from_bytes(input: &'a [u8]) -> Result<(Self, usize), ParseError> {
        let mut offset = 0;
        let total_len = input.len();

        // Check that the fixed header (id, timestamp, transaction count) is present.
        if total_len < offset + 16 {
            return Err(ParseError::UnexpectedEOF);
        }

        // Read the number of transactions (4 bytes, little-endian)
        let num_tx = u32::from_le_bytes(input[offset + 12..offset + 16].try_into().unwrap());
        offset += 16;

        // Read each transaction
        for _ in 0..num_tx {
            // Read the transaction length (4 bytes)
            if total_len < offset + 4 {
                return Err(ParseError::UnexpectedEOF);
            }
            let tx_len = u32::from_le_bytes(input[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;

            // Verify that the transaction data is available
            if total_len < offset + tx_len {
                return Err(ParseError::UnexpectedEOF);
            }
            offset += tx_len;
        }

        offset += 32;

        if total_len < offset {
            return Err(ParseError::UnexpectedEOF);
        }

        Ok((
            RefBlock {
                data: &input[..offset],
            },
            offset,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{make_test_block, ToBytes};
    use memmap2::MmapOptions;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_ref_block_from_bytes_valid() {
        let block = make_test_block();
        let buf = block.to_bytes();

        let (decoded_block, consumed) =
            RefBlock::from_bytes(&buf).expect("Block should be parsed successfully");
        assert_eq!(decoded_block.id(), block.id());
        assert_eq!(decoded_block.timestamp(), block.timestamp());
        assert_eq!(
            decoded_block.transactions().len(),
            block.transactions().len()
        );
        for (ref_tx, tx) in std::iter::zip(
            block.transactions().iter(),
            decoded_block.transactions().iter(),
        ) {
            assert_eq!(ref_tx.data(), tx.data());
        }
        assert_eq!(decoded_block.prev_hash(), block.prev_hash());
        assert_eq!(consumed, buf.len());
    }

    #[test]
    fn test_ref_block_from_bytes_insufficient_data() {
        let buf = vec![0u8; 10];
        let res = RefBlock::from_bytes(&buf);
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_block_from_mmap() {
        let mut tmpfile = NamedTempFile::new().expect("Failed to create temporary file");
        let mut buf = Vec::new();
        buf.extend_from_slice(&42u32.to_le_bytes());
        buf.extend_from_slice(&123456789u64.to_le_bytes());
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&4u32.to_le_bytes());
        buf.extend_from_slice(&[1, 2, 3, 4]);
        buf.extend_from_slice(&[255u8; 32]);

        tmpfile
            .write_all(&buf)
            .expect("Failed to write to temporary file");

        let mmap = unsafe {
            MmapOptions::new()
                .map(tmpfile.as_file())
                .expect("Failed to map the file")
        };

        let (block_ref, consumed) =
            RefBlock::from_bytes(&mmap).expect("Block should be parsed from mmap");
        assert_eq!(block_ref.id(), 42);
        assert_eq!(block_ref.timestamp(), 123456789);
        assert_eq!(block_ref.transactions().len(), 1);
        let transactions = block_ref.transactions();
        let mut iter = transactions.iter();
        assert_eq!(iter.next().unwrap().data(), &[1, 2, 3, 4]);
        assert_eq!(block_ref.prev_hash(), &[255u8; 32]);
        assert_eq!(consumed, mmap.len());
    }
}
