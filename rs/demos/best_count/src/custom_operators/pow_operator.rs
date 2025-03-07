use crate::custom_operators::log2::*;
use crate::operator::*;
use crate::stack::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct PowOperator {
    pub index: usize,
}

impl Operator for PowOperator {
    fn cardinality(&self) -> u8 {
        2
    }

    fn priority(&self) -> u8 {
        4
    }

    fn symbol(&self) -> String {
        "^".into()
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
        (Self::eval(a, b), false)
    }

    fn index(&self) -> usize {
        self.index
    }

    fn string_on_stack(self: Rc<Self>, stack: &mut Vec<(String, Rc<dyn Operator>)>) {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.push((
            std::format!("{}^{}", self.prepare(a), self.prepare(b)),
            self.clone(),
        ));
    }
}

impl PowOperator {
    fn check(a: u64, b: u64) -> bool {
        let log2a = log2(a);
        let log2b = log2(b);
        log2(log2a) + log2b < MAXDIGITNUMBER && b * log2a < MAXDIGITNUMBER
    }

    fn eval(mut a: u64, mut b: u64) -> u64 {
        match a {
            0 => 0,
            1 => 1,
            _ => {
                let mut r = 1;
                while b > 0 {
                    if b % 2 == 1 {
                        r *= a;
                    }
                    a *= a;
                    b >>= 1;
                }
                r
            }
        }
    }
}
