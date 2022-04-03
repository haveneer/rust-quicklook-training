use crate::operator::Kind;
use crate::operator::Operator;

pub struct Stack<'a> {
    data: Vec<(u64, bool)>,
    old_data: Vec<(u64, bool)>,
    stacked_data: u8,
    stacked_operators: Vec<&'a dyn Operator>,
    operator_usage: Vec<usize>,
}

impl<'a> Stack<'a> {
    pub fn new(len: usize) -> Self {
        Stack {
            data: vec![],
            old_data: vec![],
            stacked_data: 0,
            stacked_operators: vec![],
            operator_usage: vec![0; len],
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn back_replay(&mut self) -> &'a dyn Operator {
        let back_op = self.stacked_operators.pop().unwrap();
        self.data.pop();
        if back_op.kind() == Kind::Data {
            self.stacked_data -= 1;
        } else {
            for _ in 0..back_op.cardinality() {
                self.data.push(self.old_data.pop().unwrap()); // check_stack has been done before
            }
        }
        self.operator_usage[back_op.index()] -= 1;
        back_op
    }

    pub fn apply_operator(&mut self, op: &'a dyn Operator) {
        // println!("BEFORE AFTER => {:?}", self.data);
        // println!("Apply operator {}", op.symbol());
        let new_value = op.eval_on_stack(&self);

        for _ in 0..op.cardinality() {
            self.old_data.push(self.data.pop().unwrap()); // check_stack has been done before
        }
        if op.kind() == Kind::Data {
            self.stacked_data += 1;
        }
        self.data.push(new_value);
        self.stacked_operators.push(op);
        self.operator_usage[op.index()] += 1;
        // println!("DATA AFTER => {:?}\n", self.data);
    }

    pub fn result(&self) -> Option<StackResult> {
        if self.len() != 1 {
            None
        } else {
            self.data.last().map(|&x| StackResult {
                operators: &self.stacked_operators,
                value: x.0,
            })
        }
    }

    pub fn is_used(&self, op: &dyn Operator) -> bool {
        self.operator_usage[op.index()] != 0
    }

    // pub fn get_last_operator(&self) -> Option<&'a (dyn Operator + 'a)> {
    //     self.stacked_operators.last()
    // }
    // 
    pub fn get_data(&self, pos: usize) -> Option<u64> {
        self.data.get(self.data.len() - pos - 1).map(|v| v.0)
    }

    pub fn data_count(&self) -> u8 {
        self.stacked_data
    }
}


pub struct StackResult<'a> {
    pub operators: &'a Vec<&'a dyn Operator>,
    pub value: u64,
}


impl<'a> ToString for StackResult<'a> {
    fn to_string(&self) -> String {
        let mut string_stack = Vec::<(String, &'a dyn Operator)>::new();
        for &op in self.operators.iter() {
            op.string_on_stack(&mut string_stack);
        }

        string_stack
            .into_iter()
            .rev()
            .map(|s| s.0.to_string())
            .collect::<Vec<String>>()
            .join("; ")
    }
}
