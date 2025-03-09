use crate::operator::Kind;
use crate::operator::Operator;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

pub struct Stack {
    data: Vec<(u64, bool)>,
    old_data: Vec<(u64, bool)>,
    stacked_data: u8,
    stacked_operators: Vec<Rc<dyn Operator>>,
    operator_usage: HashMap<usize, usize>,
}

impl Stack {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Stack {
            data: vec![],
            old_data: vec![],
            stacked_data: 0,
            stacked_operators: vec![],
            operator_usage: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn back_replay(&mut self) -> Rc<dyn Operator> {
        let back_op = self.stacked_operators.pop().unwrap();
        self.data.pop();
        if back_op.kind() == Kind::Data {
            self.stacked_data -= 1;
        } else {
            for _ in 0..back_op.cardinality() {
                self.data.push(self.old_data.pop().unwrap()); // check_stack has been done before
            }
        }
        match self.operator_usage.get_mut(&back_op.index()) {
            Some(v) => *v -= 1,
            None => panic!("Should exist in back_replay"),
        }
        back_op
    }

    pub fn apply_operator(&mut self, op: &Rc<dyn Operator>) {
        // println!("BEFORE AFTER => {:?}", self.data);
        // println!("Apply operator {}", op.symbol());
        let new_value = op.eval_on_stack(self);

        for _ in 0..op.cardinality() {
            self.old_data.push(self.data.pop().unwrap()); // check_stack has been done before
        }
        if op.kind() == Kind::Data {
            self.stacked_data += 1;
        }
        self.data.push(new_value);
        self.stacked_operators.push(op.clone());
        match self.operator_usage.get_mut(&op.index()) {
            Some(v) => *v += 1,
            None => {
                self.operator_usage.insert(op.index(), 1);
            }
        }
        // println!("DATA AFTER => {:?}\n", self.data);
    }

    pub fn result(&self) -> Option<StackResult> {
        if self.len() != 1 {
            None
        } else {
            self.data.last().map(|value| StackResult {
                operators: &self.stacked_operators,
                value: &value.0,
            })
        }
    }

    pub fn is_used(&self, op: &dyn Operator) -> bool {
        self.operator_usage
            .get(&op.index())
            // .map(|&v| v != 0).unwrap_or(false)
            .is_some_and(|&v| v != 0)
    }

    pub fn get_last_operator(&self) -> Option<&Rc<dyn Operator>> {
        self.stacked_operators.last()
    }

    pub fn get_data(&self, pos: usize) -> Option<u64> {
        self.data.get(self.data.len() - pos - 1).map(|v| v.0)
    }
}

pub struct StackResult<'a> {
    pub operators: &'a Vec<Rc<dyn Operator>>,
    pub value: &'a u64,
}

impl Display for StackResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string_stack = Vec::<(String, Rc<dyn Operator>)>::new();
        for op in self.operators.iter() {
            op.clone().string_on_stack(&mut string_stack);
        }

        let str = string_stack
            .into_iter()
            .rev()
            .map(|s| s.0.to_string())
            .collect::<Vec<String>>()
            .join("; ");
        write!(f, "{}", str)
    }
}
