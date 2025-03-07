mod block_impl;
mod blockref_safe_impl;
mod blockref_unsafe_impl;

use std::ops::Deref;

/// Represents a block that borrows its data from the original byte buffer.
/// Format:
/// - 4 bytes: id (u32, little-endian)
/// - 8 bytes: timestamp (u64, little-endian)
/// - 32 bytes: previous block hash
/// - 4 bytes: number of transactions (u32)
/// - For each transaction:
///   - 4 bytes: transaction length (u32)
///   - X bytes: transaction data
#[derive(Debug, PartialEq, Eq)]
pub struct BlockRef<'a> {
    pub id: u32,
    pub timestamp: u64,
    pub transactions: Vec<TransactionRef<'a>>,
    pub prev_hash: &'a [u8; 32],
}

/// Represents a transaction as a slice of bytes.
#[derive(Debug, PartialEq, Eq)]
pub struct TransactionRef<'a> {
    pub data: &'a [u8],
}

/// Possible errors during parsing.
#[derive(Debug)]
pub enum ParseError {
    UnexpectedEOF,
    InvalidData,
}

/// Represents an owning block with its data.
#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    pub id: u32,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub prev_hash: [u8; 32],
}

/// Represents an owning transaction with its data.
#[derive(Debug, PartialEq, Eq)]
pub struct Transaction {
    pub data: Vec<u8>,
}

/// Conversion from a borrowed block (`BlockRef`) into an owning block (`Block`).
impl<'a> From<BlockRef<'a>> for Block {
    fn from(block_ref: BlockRef<'a>) -> Self {
        Block {
            id: block_ref.id,
            timestamp: block_ref.timestamp,
            transactions: block_ref
                .transactions
                .into_iter()
                .map(|tx_ref| Transaction {
                    data: tx_ref.data.to_vec(),
                })
                .collect(),
            prev_hash: *block_ref.prev_hash,
        }
    }
}

/// Conversion from an owning block (`&Block`) into a borrowed block (`BlockRef`).
/// This conversion creates a view into the data owned by `Block`.
impl<'a> From<&'a Block> for BlockRef<'a> {
    fn from(block: &'a Block) -> Self {
        BlockRef {
            id: block.id,
            timestamp: block.timestamp,
            transactions: block
                .transactions
                .iter()
                .map(|tx| TransactionRef { data: &tx.data })
                .collect(),
            prev_hash: &block.prev_hash,
        }
    }
}

impl<'a> Deref for BlockRef<'a> {
    type Target = [TransactionRef<'a>];

    fn deref(&self) -> &Self::Target {
        &self.transactions
    }
}

#[cfg(test)]
mod tests {
    use crate::{Block, BlockRef, Transaction, TransactionRef};
    use std::ops::Deref;

    #[test]
    fn test_blockref_from_block_conversion() {
        let block = Block {
            id: 55,
            timestamp: 55555555,
            transactions: vec![Transaction {
                data: vec![11, 22, 33],
            }],
            prev_hash: [9u8; 32],
        };

        let block_ref: BlockRef = (&block).into();
        assert_eq!(block_ref.id, 55);
        assert_eq!(block_ref.timestamp, 55555555);
        assert_eq!(block_ref.transactions.len(), 1);
        assert_eq!(block_ref.transactions[0].data, &[11, 22, 33]);
        assert_eq!(block_ref.prev_hash, &[9u8; 32]);
    }

    // A helper function that processes a slice of transactions.
    fn count_transactions(transactions: &[TransactionRef]) -> usize {
        transactions.len()
    }

    #[test]
    fn test_pass_blockref_as_transactions_slice() {
        // Create an owning block.
        let block = Block {
            id: 1,
            timestamp: 100,
            transactions: vec![
                Transaction {
                    data: vec![1, 2, 3],
                },
                Transaction {
                    data: vec![4, 5, 6],
                },
            ],
            prev_hash: [0u8; 32],
        };

        // Convert &Block to BlockRef using the From implementation.
        let block_ref: BlockRef = (&block).into();

        // Using our Borrow implementation on BlockRef, we can obtain a slice of TransactionRef.
        let tx_slice: &[TransactionRef] = block_ref.deref();
        assert_eq!(tx_slice.len(), 2);

        // Alternatively, pass the borrowed slice directly to a function expecting &[TransactionRef].
        let count = count_transactions(&block_ref);
        assert_eq!(count, 2);
    }
}
