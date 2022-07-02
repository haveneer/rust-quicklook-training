use crate::operator::*;
use crate::stack::*;
use std::borrow::Borrow;
use std::rc::Rc;

#[derive(Clone)]
pub struct SeqDataOperator {
    pub value: u64,
    pub prev: Option<Rc<dyn Operator>>,
    pub index: usize,
}

impl Operator for SeqDataOperator {
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
        let previous_value_is_used = self
            .prev
            .as_ref()
            .map(|x| stack.is_used(x.borrow()))
            .unwrap_or(true);
        previous_value_is_used && !stack.is_used(self)
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
