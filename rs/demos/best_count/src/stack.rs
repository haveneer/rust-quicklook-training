use std::collections::HashMap;
use std::rc::Rc;
use crate::operator::Kind;
use crate::operator::Operator;

pub struct Stack {
    pub data: Vec<(u64, bool)>,
    pub old_data: Vec<(u64, bool)>,
    pub stacked_data: u8,
    // operators: Vec<Box<dyn Operator>>,
    pub stacked_operators: Vec<Rc<dyn Operator>>,
    pub operator_usage: HashMap<usize, usize>,
}

impl Stack {
    pub fn back_replay(&mut self) -> Rc<dyn Operator> {
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

    pub fn apply_operator(&mut self, op: &Rc<dyn Operator>) {
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

    pub fn value(&self) -> u64 {
        if let Some(data) = self.data.last() {
            data.0
        } else {
            0
        }
    }

    pub fn is_used(&self, op: &dyn Operator) -> bool {
        if let Some(&v) = self.operator_usage.get(&op.index()) {
            v != 0
        } else {
            false
        }
    }

    pub fn get_data(&self, pos: usize) -> u64 {
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

