use crate::ParseError;

pub trait Block {
    fn id(&self) -> u32;
    fn timestamp(&self) -> u64;
    fn transactions(&self) -> impl Transactions;
    fn prev_hash(&self) -> &[u8; 32];
}

pub trait Transactions {
    type Transaction<'b>: Transaction
    where
        Self: 'b;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn iter(&self) -> impl Iterator<Item = Self::Transaction<'_>>;
}

pub trait Transaction {
    fn data(&self) -> &[u8];
}

pub trait FromBytes<'a> {
    fn from_bytes(input: &'a [u8]) -> Result<(Self, usize), ParseError>
    where
        Self: Sized;
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}
