mod operator;
mod stack;
mod custom_operators;

use std::collections::HashSet;
use std::rc::Rc;
use clap::Parser;
use crate::operator::Operator;
use crate::stack::Stack;
use crate::custom_operators::*;


fn compute(target: u64, operators: Vec<Rc<dyn Operator>>) -> HashSet<String> {
    let mut stack = Stack::new();
    let mut solutions = HashSet::new();

    let mut next_op: Option<&Rc<dyn Operator>> = operators.first();
    while next_op.is_some() {
        while let Some(op) = next_op {
            next_op = if op.check_stack(&stack) {
                stack.apply_operator(&op);
                operators.first()
            } else {
                operators.get(op.index() + 1)
            }
        }

        // std::println!("{}", stack.to_string());
        let is_valid = // should be more constrained
            stack.len() == 1
                && stack.value() == target;
        if is_valid {
            // println!("Solution found");
            solutions.insert(stack.to_string());
        }

        while stack.len() > 0 && next_op.is_none() {
            let old_op = stack.back_replay();
            // std::println!("Backreplay : {} [{} -> out]", stack.to_string(), old_op.symbol());
            next_op = operators.get(old_op.index() + 1);
        }
    }
    solutions
}

#[derive(Parser, Debug)]
struct Args {
    /// Number of times to greet
    #[clap(short, long, default_value_t = 4)]
    target: u64,
}

fn main() {
    let args = Args::parse();

    println!("Target = {}", args.target);

    let operators: Vec<Rc<dyn Operator>> =
        vec![
            Rc::new(UniqueDataOperator { value: 1, index: 0 }),
            Rc::new(UniqueDataOperator { value: 2, index: 1 }),
            Rc::new(UniqueDataOperator { value: 3, index: 2 }),
            Rc::new(AddOperator { index: 3 }),
        ];

    let results = compute(args.target, operators);

    if results.is_empty() {
        println!("No solution has been found");
    } else {
        println!("Found solution(s):");
        results.into_iter().for_each(|s| println!("{}", s));
    }
}