use crate::operator::*;
use crate::stack::*;

#[derive(Clone)]
pub struct DivOperator {
    pub index: usize,
}

impl Operator for DivOperator {
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
        if stack.len() < self.cardinality().into()
        { false } else {
            let b = stack.get_data(0).unwrap();
            let a = stack.get_data(1).unwrap();
            b != 0 && (a % b == 0)
        }
    }

    fn eval_on_stack(&self, stack: &Stack) -> (u64, bool) {
        let b = stack.get_data(0).unwrap();
        let a = stack.get_data(1).unwrap();
        (a / b, false)
    }

    fn index(&self) -> usize {
        self.index
    }

    fn string_on_stack(&self, stack: &mut Vec<(String, &dyn Operator)>) {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        // stack.push((std::format!("{}/{}", self.prepare(a), self.prepare_extended(b)), self));
    }
}
