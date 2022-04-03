mod operator;
pub mod stack;
mod custom_operators;

use std::collections::HashSet;
pub use crate::operator::Operator;
pub use crate::stack::Stack;
pub use crate::custom_operators::*;


pub fn compute(target: u64, operators: Vec<Box<dyn Operator>>, test: impl Fn(&Stack) -> bool) -> HashSet<String>
{
    let mut stack = Stack::new(operators.len());
    let mut solutions = HashSet::new();

    let mut next_op: Option<&Box<dyn Operator>> = operators.first();
    while let Some(op) = next_op {
        next_op = if op.check_stack(&stack) {
            stack.apply_operator(op.as_ref());
            operators.first()
        } else {
            operators.get(op.index() + 1)
        };
        
        // std::println!("{}", stack.to_string());
        if let Some(result) = stack.result() {
            if result.value == target && test(&stack) {
                // let solution_as_string = result.to_string();
                // if solutions.insert(solution_as_string.clone()) {
                //     println!("Solution[{}] {}", solutions.len(), solution_as_string);
                // }
            }
        }
        
        while stack.len() > 0 && next_op.is_none() {
            let old_op = stack.back_replay();
            // std::println!("Backreplay : {} [{} -> out]", stack.to_string(), old_op.symbol());
            next_op = operators.get(old_op.index() + 1);
        }
    }
    solutions
}