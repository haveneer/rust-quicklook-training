use crate::operator::*;
use crate::stack::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct ContactOperator {
    pub index: usize,
}

impl Operator for ContactOperator {
    fn cardinality(&self) -> u8 {
        2
    }

    fn priority(&self) -> u8 {
        0
    }

    fn symbol(&self) -> String {
        ".".into()
    }

    fn kind(&self) -> Kind {
        Kind::Operator
    }

    fn check_stack(&self, stack: &Stack) -> bool {
        stack.len() >= self.cardinality().into()
    }

    fn eval_on_stack(&self, stack: &Stack) -> (u64, bool) {
        let b = stack.get_data(0).unwrap();
        let a = stack.get_data(1).unwrap();
        (a + b, false)
    }

    fn index(&self) -> usize {
        self.index
    }

    fn string_on_stack(self: Rc<Self>, stack: &mut Vec<(String, Rc<dyn Operator>)>) {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.push((
            std::format!("{}.{}", self.prepare(a), self.prepare(b)),
            self.clone(),
        ));
    }
}
