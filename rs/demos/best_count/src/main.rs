mod operator;
mod stack;
mod custom_operators;

use std::borrow::Borrow;
use std::collections::HashSet;
use std::rc::Rc;
use clap::Parser;
use crate::operator::Operator;
use crate::stack::Stack;
use crate::custom_operators::*;


fn compute(target: u64, operators: Vec<Rc<dyn Operator>>, test: impl Fn(&Stack) -> bool) -> HashSet<String>
{
    let mut stack = Stack::new();
    let mut solutions = HashSet::new();

    let mut next_op: Option<&Rc<dyn Operator>> = operators.first();
    while let Some(op) = next_op {
        next_op = if op.check_stack(&stack) {
            stack.apply_operator(&op);
            operators.first()
        } else {
            operators.get(op.index() + 1)
        };

        // std::println!("{}", stack.to_string());
        let is_valid = stack.len() == 1 && stack.value() == target;
        if is_valid && test(&stack) { // should be more constrained using test
            let solution_as_string = stack.to_string();
            if solutions.insert(solution_as_string.clone()) {
                println!("Solution[{}] {}", solutions.len(), solution_as_string);
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

#[derive(Parser, Debug)]
struct Args {
    /// Number of times to greet
    #[clap(short, long, default_value_t = 2022)]
    target: u64,
}

fn main() {
    let args = Args::parse();

    println!("Target = {}", args.target);

    let mut operators = Vec::<Rc<dyn Operator>>::new();
    // operators.push(Rc::new(UniqueDataOperator { value: 1, index: operators.len() }));
    // operators.push(Rc::new(UniqueDataOperator { value: 2, index: operators.len() }));
    // operators.push(Rc::new(UniqueDataOperator { value: 3, index: operators.len() }));

    let mut prev: Option<Rc<dyn Operator>> = None;
    for i in 1..=8 {
        let next = Rc::new(SeqDataOperator { value: i, prev: prev.map(|o| o.clone()), index: operators.len() });
        operators.push(next.clone());
        prev = Some(next);
    }
    operators.push(Rc::new(AddOperator { index: operators.len() }));
    operators.push(Rc::new(MultOperator { index: operators.len() }));
    operators.push(Rc::new(DivOperator { index: operators.len() }));
    operators.push(Rc::new(PowOperator { index: operators.len() }));
    operators.push(Rc::new(FactorialOperator { index: operators.len() }));

    let test = {
        let op = prev.unwrap();
        move |s: &Stack| {
            s.is_used(op.borrow())
        }
    };
    let results = compute(args.target, operators, test);

    if results.is_empty() {
        println!("No solution has been found");
    } else {
        //     println!("Found solution(s):");
        //     results.into_iter().for_each(|s| println!("{}", s));
    }
}