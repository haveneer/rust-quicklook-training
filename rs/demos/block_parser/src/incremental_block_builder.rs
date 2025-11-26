use crate::{ParseError, RefBlock};
use std::marker::PhantomData;

pub struct New;
pub struct WithId;
pub struct WithTimestamp;

pub struct IncrementalBlockBuilder<'a, Status = New> {
    data: &'a mut [u8],
    offset: usize,
    transaction_count: usize,
    status: PhantomData<Status>,
}

impl<'a> IncrementalBlockBuilder<'a, New> {
    pub fn new(data: &'a mut [u8]) -> Result<IncrementalBlockBuilder<'a, New>, ParseError> {
        if data.len() < 16 {
            return Err(ParseError::UnexpectedEOF);
        }
        Ok(Self {
            data,
            offset: 16, // position after transaction count
            transaction_count: 0,
            status: PhantomData,
        })
    }

    pub fn with_id(
        &'a mut self,
        id: u32,
    ) -> Result<IncrementalBlockBuilder<'a, WithId>, ParseError> {
        self.data[0..4].copy_from_slice(&id.to_le_bytes());
        Ok(IncrementalBlockBuilder::<WithId> {
            data: self.data,
            offset: self.offset,
            transaction_count: self.transaction_count,
            status: PhantomData,
        })
    }
}

impl<'a> IncrementalBlockBuilder<'a, WithId> {
    pub fn with_timestamp(
        &'a mut self,
        timestamp: u64,
    ) -> Result<IncrementalBlockBuilder<'a, WithTimestamp>, ParseError> {
        self.data[4..12].copy_from_slice(&timestamp.to_le_bytes());
        Ok(IncrementalBlockBuilder::<WithTimestamp> {
            data: self.data,
            offset: self.offset,
            transaction_count: self.transaction_count,
            status: PhantomData,
        })
    }
}

impl<'a> IncrementalBlockBuilder<'a, WithTimestamp> {
    pub fn add_transaction(&mut self, transaction: &[u8]) -> Result<(), ParseError> {
        let trlen = transaction.len();
        if self.data.len() < self.offset + 4 + trlen {
            return Err(ParseError::UnexpectedEOF);
        }
        self.data[self.offset..self.offset + 4]
            .copy_from_slice(&(trlen as u32).to_le_bytes());
        self.offset += 4;
        self.data[self.offset..self.offset+trlen].copy_from_slice(&transaction);
        self.offset += trlen;
        self.transaction_count += 1;
        Ok(())
    }

    pub fn with_prev_hash(&mut self, hash: &[u8; 32]) -> Result<(RefBlock, usize), ParseError> {
        if self.data.len() < self.offset + 32 {
            return Err(ParseError::UnexpectedEOF);
        }
        self.data[self.offset..self.offset + 32].copy_from_slice(hash);
        self.offset += 32;
        self.data[12..16].copy_from_slice(&(self.transaction_count as u32).to_le_bytes());
        Ok((
            RefBlock {
                data: self.data[..self.offset].into(),
            },
            self.offset,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::incremental_block_builder::IncrementalBlockBuilder;
    use crate::{make_test_block, Block, ToBytes, Transaction};
    use crate::block_interfaces::Transactions;

    #[test]
    fn block_to_refblock_via_incremental_builder() {
        let block = make_test_block();
        let mut data = [0; 1024];
        let mut builder = IncrementalBlockBuilder::new(&mut data).unwrap();
        let mut builder = builder.with_id(block.id()).unwrap();
        let mut builder = builder.with_timestamp(block.timestamp()).unwrap();
        for t in block.transactions().iter() {
             builder.add_transaction(t.data()).unwrap();
        }
        let (ref_block, size) = builder.with_prev_hash(block.prev_hash()).unwrap();
        let data = &data[..size];
        assert_eq!(data, block.to_bytes());
    }
}
