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
        loop {
            if next_op.unwrap().check_stack(&stack) {
                stack.apply_operator(&next_op.unwrap());
                next_op = operators.first();
            } else {
                next_op = operators.get(next_op.unwrap().index() + 1);
                if next_op.is_none() {
                    break;
                }
            }
        }

        // std::println!("{}", stack.to_string());

        let is_valid = stack.len() == 1 && stack.value() == target; // should be more constrained
        if is_valid {
            // println!("Solution found");
            solutions.insert(stack.to_string());
        }

        if stack.len() > 0 {
            loop {
                let old_op = stack.back_replay();
                // std::println!("Backreplay : {} [{} -> out]", stack.to_string(), old_op.symbol());
                next_op = operators.get(old_op.index() + 1);
                if next_op.is_some() || stack.len() == 0
                { break; }
            }
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