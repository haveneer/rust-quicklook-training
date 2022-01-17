use std::rc::Rc;
use crate::operator::*;
use crate::stack::*;

#[derive(Clone)]
pub struct FactorialOperator {
    pub index: usize,
}

impl Operator for FactorialOperator {
    fn cardinality(&self) -> u8 {
        1
    }

    fn priority(&self) -> u8 {
        2
    }

    fn symbol(&self) -> String {
        "!".into()
    }

    fn kind(&self) -> Kind {
        Kind::Operator
    }

    fn check_stack(&self, stack: &Stack) -> bool {
        if stack.len() < self.cardinality().into() {
            false
        } else {
            let n = stack.get_data(0).unwrap();
            n > 2 && n <= 12
        }
    }

    fn eval_on_stack(&self, stack: &Stack) -> (u64, bool) {
        let n = stack.get_data(0).unwrap();
        (Self::eval(n), false)
    }

    fn index(&self) -> usize {
        self.index
    }

    fn string_on_stack(self: Rc<Self>, stack: &mut Vec<(String, Rc<dyn Operator>)>) {
        let n = stack.pop().unwrap();
        stack.push((std::format!("{}!", self.prepare(n)), self.clone()));
    }
}

impl FactorialOperator {
    fn eval(n: u64) -> u64 {
        let mut r = 1;
        for i in 2..=n { r *= i; }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_eval() {
        assert_eq!(FactorialOperator::eval(0), 1);
        assert_eq!(FactorialOperator::eval(1), 1);
        assert_eq!(FactorialOperator::eval(2), 2);
        assert_eq!(FactorialOperator::eval(3), 6);
    }
}