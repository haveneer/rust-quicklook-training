use crate::operator::*;
use crate::stack::*;

#[derive(Clone)]
pub struct UniqueDataOperator {
    pub value: u64,
    pub index: usize,
}

impl Operator for UniqueDataOperator {
    fn cardinality(&self) -> u8 {
        0
    }

    fn priority(&self) -> u8 {
        0
    }

    fn symbol(&self) -> String {
        self.value.to_string()
    }

    fn kind(&self) -> Kind {
        Kind::Data
    }

    fn check_stack(&self, stack: &Stack) -> bool {
        !stack.is_used(self)
    }

    fn eval_on_stack(&self, stack: &Stack) -> (u64, bool) {
        (self.value, true)
    }

    fn index(&self) -> usize {
        self.index
    }

    fn string_on_stack(&self, stack: &mut Vec<String>) {
        stack.push(self.value.to_string());
    }
}

