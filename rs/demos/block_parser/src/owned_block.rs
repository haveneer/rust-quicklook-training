use crate::block_interfaces::{Block, FromBytes, ToBytes, Transaction, Transactions};
use crate::{ParseError, RefBlock};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

/// Represents an owning block with its data.
/// Format:
/// - 4 bytes: id (u32, little-endian)
/// - 8 bytes: timestamp (u64, little-endian)
/// - 4 bytes: number of transactions (u32)
/// - For each transaction:
///   - 4 bytes: transaction length (u32)
///   - X bytes: transaction data
/// - 32 bytes: previous block hash
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedBlock {
    id: u32,
    timestamp: u64,
    transactions: Vec<OwnedTransaction>,
    prev_hash: [u8; 32],
}

impl OwnedBlock {
    pub fn new(
        id: u32,
        timestamp: u64,
        transactions: Vec<OwnedTransaction>,
        prev_hash: [u8; 32],
    ) -> Self {
        Self {
            id,
            timestamp,
            transactions,
            prev_hash,
        }
    }
}

/// Represents an owning transaction with its data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedTransaction {
    pub(crate) data: Vec<u8>,
}

impl Transaction for OwnedTransaction {
    fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}

impl Block for OwnedBlock {
    fn id(&self) -> u32 {
        self.id
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn transactions(&self) -> impl Transactions {
        self.transactions.as_slice()
    }

    fn prev_hash(&self) -> &[u8; 32] {
        &self.prev_hash
    }
}

impl Transaction for &OwnedTransaction {
    fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Transactions for &[OwnedTransaction] {
    type Transaction<'b>
        = &'b OwnedTransaction
    where
        Self: 'b;

    fn len(&self) -> usize {
        <[OwnedTransaction]>::len(self)
    }

    fn iter(&self) -> impl Iterator<Item = Self::Transaction<'_>> {
        <[OwnedTransaction]>::iter(self)
    }
}

impl ToBytes for OwnedBlock {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.id.to_le_bytes());
        buf.extend_from_slice(&self.timestamp.to_le_bytes());
        let num_tx = self.transactions.len() as u32;
        buf.extend_from_slice(&num_tx.to_le_bytes());
        for tx in &self.transactions {
            let tx_len = tx.data.len() as u32;
            buf.extend_from_slice(&tx_len.to_le_bytes());
            buf.extend_from_slice(&tx.data);
        }
        buf.extend_from_slice(&self.prev_hash);
        buf
    }
}

impl<'a> FromBytes<'a> for OwnedBlock {
    fn from_bytes(input: &'a [u8]) -> Result<(Self, usize), ParseError> {
        let (block_ref, consumed) = RefBlock::from_bytes(input)?;
        Ok((block_ref.into(), consumed))
    }
}

/// Conversion from a borrowed block (`RefBlock`) into an owning block (`OwnedBlock`).
impl<'a> From<RefBlock<'a>> for OwnedBlock {
    fn from(block_ref: RefBlock<'a>) -> Self {
        OwnedBlock {
            id: block_ref.id(),
            timestamp: block_ref.timestamp(),
            transactions: block_ref
                .transactions()
                .iter()
                .map(|tx_ref| OwnedTransaction {
                    data: tx_ref.data().to_vec(),
                })
                .collect(),
            prev_hash: *block_ref.prev_hash(),
        }
    }
}

pub fn make_test_block() -> OwnedBlock {
    OwnedBlock {
        id: 10,
        timestamp: 987654321,
        transactions: vec![
            OwnedTransaction {
                data: vec![5, 6, 7],
            },
            OwnedTransaction { data: vec![8, 9] },
        ],
        prev_hash: [1u8; 32],
    }
}

pub fn make_test_block_with_seed(seed: u64) -> OwnedBlock {
    let mut r = StdRng::seed_from_u64(seed);
    OwnedBlock {
        id: r.gen(),
        timestamp: r.gen(),
        transactions: make_test_transactions_with_seed(seed),
        prev_hash: r.gen(),
    }
}

pub fn make_test_transactions_with_seed(seed: u64) -> Vec<OwnedTransaction> {
    let mut r = StdRng::seed_from_u64(seed);

    let mut transactions = Vec::new();
    for _ in 0..r.gen_range(0..10) {
        let n = r.gen_range(0..20);
        let mut data = vec![0; n];
        data.iter_mut().for_each(|x| *x = r.gen());
        // TODO replace dynamic allocation with a pre-allocated buffer
        transactions.push(OwnedTransaction { data })
    }
    transactions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RefBlock;

    #[test]
    fn test_round_trip_block() {
        // Create an owning block.
        let block = make_test_block();
        let bytes = block.to_bytes();
        let (block_ref, consumed) = RefBlock::from_bytes(&bytes).expect("Parsing should succeed");
        assert_eq!(consumed, bytes.len());
        let block_converted: OwnedBlock = block_ref.into();
        assert_eq!(block, block_converted);
    }

    #[test]
    fn test_multiple_decode() {
        let blocks = (0..10)
            .map(|i| make_test_block_with_seed(i))
            .collect::<Vec<_>>();

        let mut many_block_bytes = Vec::new();
        for b in blocks.iter() {
            many_block_bytes.extend_from_slice(b.to_bytes().as_slice())
        }

        let mut offset = 0;
        for ref_block in blocks.iter() {
            let (block_ref, consumed) =
                RefBlock::from_bytes(&many_block_bytes[offset..]).expect("Parsing should succeed");
            assert_eq!(consumed, ref_block.to_bytes().len());
            assert_eq!(OwnedBlock::from(block_ref), *ref_block);
            offset += consumed;
        }
    }

    #[test]
    fn show_encoded_test_block() {
        let v = make_test_block();
        println!("{:?}", v);
        println!("{:?}", v.to_bytes());
    }

    #[test]
    fn test_to_bytes() {
        let mut buf = Vec::new();
        buf.extend_from_slice(&10u32.to_le_bytes()); // id
        buf.extend_from_slice(&987654321u64.to_le_bytes()); // timestamp

        buf.extend_from_slice(&2u32.to_le_bytes()); // transaction count

        buf.extend_from_slice(&3u32.to_le_bytes()); // first transaction on 3 bytes
        buf.extend_from_slice(&[5, 6, 7]);

        buf.extend_from_slice(&2u32.to_le_bytes()); // second transaction on 2 bytes
        buf.extend_from_slice(&[8, 9]);

        buf.extend_from_slice(&[1u8; 32]); // hash

        assert_eq!(buf.len(), 61);

        let (block, offset) = OwnedBlock::from_bytes(&buf).unwrap();
        assert_eq!(block.id(), 10);
        assert_eq!(block.timestamp(), 987654321);
        let transactions = block.transactions();
        assert_eq!(transactions.len(), 2);
        let mut iter = transactions.iter();
        assert_eq!(iter.next().unwrap().data(), &[5, 6, 7]);
        assert_eq!(iter.next().unwrap().data(), &[8, 9]);
        assert!(iter.next().is_none());
        assert_eq!(block.prev_hash(), &[1u8; 32]);
        assert_eq!(offset, buf.len());
        assert_eq!(block.to_bytes(), buf.as_slice());
    }
}
