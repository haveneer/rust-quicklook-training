use crate::operator::*;
use crate::stack::*;
use std::rc::Rc;

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

    fn eval_on_stack(&self, _stack: &Stack) -> (u64, bool) {
        (self.value, true)
    }

    fn index(&self) -> usize {
        self.index
    }

    fn string_on_stack(self: Rc<Self>, stack: &mut Vec<(String, Rc<dyn Operator>)>) {
        stack.push((self.value.to_string(), self.clone()));
    }
}
