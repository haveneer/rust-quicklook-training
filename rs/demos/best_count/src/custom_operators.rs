mod log2;

pub mod add_operator;
pub mod concat_operator;
pub mod div_operator;
pub mod factorial_operator;
pub mod mult_operator;
pub mod pow_operator;
pub mod seq_data_operator;
pub mod sub_operator;
pub mod unique_data_operator;

pub use add_operator::AddOperator;
pub use div_operator::DivOperator;
pub use factorial_operator::FactorialOperator;
pub use mult_operator::MultOperator;
pub use pow_operator::PowOperator;
pub use seq_data_operator::SeqDataOperator;
pub use unique_data_operator::UniqueDataOperator;

use crate::operator::*;
use std::fmt;

impl fmt::Debug for dyn Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Operator")
            .field("symbol", &self.symbol())
            .finish()
    }
}
