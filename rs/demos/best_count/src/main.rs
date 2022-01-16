use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use clap::Parser;

#[derive(PartialEq)]
enum Kind { Data, Unary, Binary, Function }

struct Stack {
    data: Vec<(u64, bool)>,
    old_data: Vec<(u64, bool)>,
    stacked_data: u8,
    // operators: Vec<Box<dyn Operator>>,
    stacked_operators: Vec<Rc<dyn Operator>>,
    operator_usage: HashMap<usize, usize>,
}


trait Operator /* : OperatorClone */ {
    fn cardinality(&self) -> u8;
    fn priority(&self) -> u8;
    fn symbol(&self) -> String;
    fn kind(&self) -> Kind;
    fn check_stack(&self, stack: &Stack) -> bool;
    fn eval_on_stack(&self, stack: &Stack) -> (u64, bool);
    fn index(&self) -> usize;
    fn string_on_stack(&self, stack: &mut Vec<String>);
}
// 
// trait OperatorClone {
//     fn clone_boxed(&self) -> Box<dyn Operator>;
// }
// 
// impl<T> OperatorClone for T
//     where T: 'static + Operator + Clone {
//     fn clone_boxed(&self) -> Box<dyn Operator> {
//         Box::new(self.clone())
//     }
// }
// 
// impl Clone for Box<dyn Operator> {
//     fn clone(&self) -> Box<dyn Operator> {
//         self.clone_boxed()
//     }
// }

#[derive(Clone)]
struct UniqueDataOperator {
    value: u64,
    index: usize,
}

impl Operator for UniqueDataOperator {
    fn cardinality(&self) -> u8 {
        0
    }

    fn priority(&self) -> u8 {
        0
    }

    fn symbol(&self) -> String {
        self.value.to_string()
    }

    fn kind(&self) -> Kind {
        Kind::Data
    }

    fn check_stack(&self, stack: &Stack) -> bool {
        !stack.is_used(self)
    }

    fn eval_on_stack(&self, stack: &Stack) -> (u64, bool) {
        (self.value, true)
    }

    fn index(&self) -> usize {
        self.index
    }

    fn string_on_stack(&self, stack: &mut Vec<String>) {
        stack.push(self.value.to_string());
    }
}

#[derive(Clone)]
struct AddOperator {
    index: usize,
}

impl Operator for AddOperator {
    fn cardinality(&self) -> u8 {
        2
    }

    fn priority(&self) -> u8 {
        10
    }

    fn symbol(&self) -> String {
        "+".into()
    }

    fn kind(&self) -> Kind {
        Kind::Binary
    }

    fn check_stack(&self, stack: &Stack) -> bool {
        stack.data.len() >= self.cardinality().into()
    }

    fn eval_on_stack(&self, stack: &Stack) -> (u64, bool) {
        let b = stack.get_data(0);
        let a = stack.get_data(1);
        (a + b, false)
    }

    fn index(&self) -> usize {
        self.index
    }

    fn string_on_stack(&self, stack: &mut Vec<String>) {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.push(std::format!("{} + {}", a, b));
    }
}

impl Stack {
    fn back_replay(&mut self) -> Rc<dyn Operator> {
        let back_op = self.stacked_operators.pop().unwrap();
        self.data.pop();
        if back_op.kind() == Kind::Data {
            self.stacked_data -= 1;
        } else {
            for i in 0..back_op.cardinality() {
                self.data.push(self.old_data.pop().unwrap()); // check_stack has been done before
            }
        }
        match self.operator_usage.get_mut(&back_op.index()) {
            Some(v) => *v -= 1,
            None => panic!("Should exist in back_replay")
        }
        back_op
    }

    fn apply_operator(&mut self, op: &Rc<dyn Operator>) {
        // println!("Apply operator {}", op.symbol());
        let new_value = op.eval_on_stack(&self);

        for i in 0..op.cardinality() {
            self.old_data.push(self.data.pop().unwrap()); // check_stack has been done before
        }
        if op.kind() == Kind::Data {
            self.stacked_data += 1;
        }
        self.data.push(new_value);
        self.stacked_operators.push(op.clone());
        match self.operator_usage.get_mut(&op.index()) {
            Some(v) => *v += 1,
            None => { self.operator_usage.insert(op.index(), 1); }
        }
    }

    fn get_operator(&self) -> Option<&Rc<dyn Operator>> {
        self.stacked_operators.last()
    }

    fn value(&self) -> u64 {
        if let Some(data) = self.data.last() {
            data.0
        } else {
            0
        }
    }

    fn is_used(&self, op: &dyn Operator) -> bool {
        if let Some(&v) = self.operator_usage.get(&op.index()) {
            v != 0
        } else {
            false
        }
    }

    fn get_data(&self, pos: usize) -> u64 {
        self.data.get(self.data.len() - pos - 1).unwrap().0
    }
}

impl ToString for Stack {
    fn to_string(&self) -> String {
        let mut string_stack = Vec::<String>::new();
        for op in self.stacked_operators.iter() {
            op.string_on_stack(&mut string_stack);
        }
        let stack_summary = string_stack
            .into_iter()
            .rev()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("; ");
        std::format!("{} = {}", self.value(), stack_summary)
    }
}


fn compute(target: u64, operators: Vec<Rc<dyn Operator>>) -> HashSet<String> {
    let mut stack = Stack {
        data: vec![],
        old_data: vec![],
        stacked_data: 0,
        stacked_operators: vec![],
        operator_usage: HashMap::new(),
    };

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

        let is_valid = stack.data.len() == 1 && stack.value() == target; // should be more constrained
        if is_valid {
            // println!("Solution found");
            solutions.insert(stack.to_string());
        }

        if stack.data.len() > 0 {
            loop {
                let old_op = stack.back_replay();
                // std::println!("Backreplay : {} [{} -> out]", stack.to_string(), old_op.symbol());
                next_op = operators.get(old_op.index() + 1);
                if next_op.is_some() || stack.data.len() == 0
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