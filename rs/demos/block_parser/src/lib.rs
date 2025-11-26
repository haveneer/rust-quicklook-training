mod block_interfaces;
pub mod block_iter;
mod owned_block;
mod process;
mod ref_block;
pub mod async_tools;
mod incremental_block_builder;

pub use ref_block::{RefBlock, RefTransaction, RefTransactions};
pub use incremental_block_builder::IncrementalBlockBuilder;
pub use block_interfaces::{Block, FromBytes, ToBytes, Transaction, Transactions};
pub use owned_block::{
    make_test_block, make_test_block_with_seed, make_test_transactions_with_seed,
};
pub use owned_block::{OwnedBlock, OwnedTransaction};

/// Possible errors during parsing.
#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedEOF,
    InvalidData,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedEOF => write!(f, "Fin de données inattendue lors du parsing"),
            ParseError::InvalidData => write!(f, "Conversion de données invalide"),
        }
    }
}

impl std::error::Error for ParseError {}
