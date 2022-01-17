use std::rc::Rc;
use crate::operator::*;
use crate::stack::*;
use crate::custom_operators::log2::*;

#[derive(Clone)]
pub struct MultOperator {
    pub index: usize,
}

impl Operator for MultOperator {
    fn cardinality(&self) -> u8 {
        2
    }

    fn priority(&self) -> u8 {
        8
    }

    fn symbol(&self) -> String {
        "*".into()
    }

    fn kind(&self) -> Kind {
        Kind::Operator
    }

    fn check_stack(&self, stack: &Stack) -> bool {
        if stack.len() < self.cardinality().into() {
            false
        } else {
            let b = stack.get_data(0).unwrap();
            let a = stack.get_data(1).unwrap();
            Self::check(a, b)
        }
    }

    fn eval_on_stack(&self, stack: &Stack) -> (u64, bool) {
        let b = stack.get_data(0).unwrap();
        let a = stack.get_data(1).unwrap();
        (a * b, false)
    }

    fn index(&self) -> usize {
        self.index
    }

    fn string_on_stack(self: Rc<Self>, stack: &mut Vec<(String, Rc<dyn Operator>)>) {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.push((std::format!("{} * {}", self.prepare(a), self.prepare(b)), self.clone()));
    }
}

impl MultOperator {
    fn check(a: u64, b: u64) -> bool {
        log2(a) + log2(b) < MAXDIGITNUMBER
    }
}