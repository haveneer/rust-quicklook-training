mod log2;

pub mod seq_data_operator;
pub mod unique_data_operator;
pub mod add_operator;
pub mod mult_operator;
pub mod div_operator;
pub mod pow_operator;
pub mod factorial_operator;

pub use unique_data_operator::UniqueDataOperator;
pub use seq_data_operator::SeqDataOperator;
pub use add_operator::AddOperator;
pub use mult_operator::MultOperator;
pub use div_operator::DivOperator;
pub use pow_operator::PowOperator;
pub use factorial_operator::FactorialOperator;

use std::fmt;
use crate::operator::*;

impl fmt::Debug for dyn Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Operator")
            .field("symbol", &self.symbol())
            .finish()
    }
}
