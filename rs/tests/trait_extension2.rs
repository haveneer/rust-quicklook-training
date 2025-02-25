use std::borrow::{Borrow, BorrowMut};

struct Parameters {
    internal_fields: Vec<String>,
}

impl Parameters {
    fn new() -> Self {
        Parameters {
            internal_fields: Vec::new(),
        }
    }
}

pub trait ParameterExt: BorrowMut<Parameters> {
    fn add_protocol(&mut self, name: &str, token0: &str, token1: &str) {
        self.borrow_mut().internal_fields.push(name.to_string());
        self.borrow_mut().internal_fields.push(token0.to_string());
        self.borrow_mut().internal_fields.push(token1.to_string());
    }
}

impl ParameterExt for Parameters {}

fn main() {
    let mut parameters = Parameters::new();
    parameters.add_protocol("UniswapV3", "USDC", "DAI");
}

#[test]
fn test() {
    main()
}
