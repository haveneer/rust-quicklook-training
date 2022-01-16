use crate::stack::Stack;

#[derive(PartialEq)]
pub enum Kind { Data, Operator }

pub trait Operator /* : OperatorClone */ {
    fn cardinality(&self) -> u8;
    fn priority(&self) -> u8;
    fn symbol(&self) -> String;
    fn kind(&self) -> Kind;
    fn check_stack(&self, stack: &Stack) -> bool;
    fn eval_on_stack(&self, stack: &Stack) -> (u64, bool);
    fn index(&self) -> usize;
    fn string_on_stack(&self, stack: &mut Vec<String>);
}
