use crate::stack::Stack;

#[derive(PartialEq)]
pub enum Kind { Data, Operator }

pub trait Operator {
    fn cardinality(&self) -> u8;
    fn priority(&self) -> u8;
    fn symbol(&self) -> String;
    fn kind(&self) -> Kind;
    fn check_stack(&self, stack: &Stack) -> bool;
    fn eval_on_stack(&self, stack: &Stack) -> (u64, bool);
    fn index(&self) -> usize;
    fn string_on_stack(&self, stack: &mut Vec<(String, &dyn Operator)>);

    fn as_operator(&self) -> &dyn Operator where Self: Sized {
        self
    }
    fn prepare(&self, x: (String, &dyn Operator)) -> String {
        let protect = x.1.priority() > self.priority();
        if protect {
            format!("({})", x.0)
        } else {
            x.0
        }
    }

    fn prepare_extended(&self, x: (String, &dyn Operator)) -> String {
        let protect = x.1.priority() >= self.priority();
        if protect {
            format!("({})", x.0)
        } else {
            x.0
        }
    }
}
