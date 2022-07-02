use crate::stack::Stack;
use std::rc::Rc;

#[derive(PartialEq)]
pub enum Kind {
    Data,
    Operator,
}

pub trait Operator {
    fn cardinality(&self) -> u8;
    fn priority(&self) -> u8;
    fn symbol(&self) -> String;
    fn kind(&self) -> Kind;
    fn check_stack(&self, stack: &Stack) -> bool;
    fn eval_on_stack(&self, stack: &Stack) -> (u64, bool);
    fn index(&self) -> usize;
    fn string_on_stack(self: Rc<Self>, stack: &mut Vec<(String, Rc<dyn Operator>)>);

    fn prepare(&self, x: (String, Rc<dyn Operator>)) -> String {
        let protect = x.1.priority() > self.priority();
        if protect {
            format!("({})", x.0)
        } else {
            x.0
        }
    }

    fn prepare_extended(&self, x: (String, Rc<dyn Operator>)) -> String {
        let protect = x.1.priority() >= self.priority();
        if protect {
            format!("({})", x.0)
        } else {
            x.0
        }
    }
}
