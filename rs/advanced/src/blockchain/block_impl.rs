use crate::{Block, BlockRef, ParseError};

impl Block {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.id.to_le_bytes());
        buf.extend_from_slice(&self.timestamp.to_le_bytes());
        let num_tx = self.transactions.len() as u32;
        buf.extend_from_slice(&self.prev_hash);
        buf.extend_from_slice(&num_tx.to_le_bytes());
        for tx in &self.transactions {
            let tx_len = tx.data.len() as u32;
            buf.extend_from_slice(&tx_len.to_le_bytes());
            buf.extend_from_slice(&tx.data);
        }
        buf
    }

    /// Parses an owning block from a byte slice.
    /// Internally, it uses `BlockRef::from_bytes` and converts it into an owning `Block`.
    pub fn from_bytes(input: &[u8]) -> Result<(Self, usize), ParseError> {
        let (block_ref, consumed) = BlockRef::from_bytes(input)?;
        Ok((block_ref.into(), consumed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Transaction;

    #[test]
    fn test_round_trip_block() {
        // Create an owning block.
        let block = Block {
            id: 10,
            timestamp: 987654321,
            transactions: vec![
                Transaction {
                    data: vec![5, 6, 7],
                },
                Transaction { data: vec![8, 9] },
            ],
            prev_hash: [1u8; 32],
        };

        let bytes = block.to_bytes();
        let (block_ref, consumed) =
            BlockRef::from_bytes(&bytes).expect("Parsing should succeed");
        assert_eq!(consumed, bytes.len());

        let block_converted: Block = block_ref.into();
        assert_eq!(block, block_converted);
    }
}
