pub use best_count::stack::StackResult;
use best_count::*;
use std::rc::Rc;

#[test]
fn expression_display() {
    let mut operators: Vec<Rc<dyn Operator>> = Vec::new();
    operators.resize(12, Rc::new(UniqueDataOperator { value: 0, index: 0 }));
    operators[1] = Rc::new(UniqueDataOperator { value: 1, index: 0 });
    operators[2] = Rc::new(UniqueDataOperator { value: 2, index: 0 });
    operators[3] = Rc::new(UniqueDataOperator { value: 3, index: 0 });
    operators[4] = Rc::new(UniqueDataOperator { value: 4, index: 0 });
    operators[5] = Rc::new(UniqueDataOperator { value: 5, index: 0 });
    operators[6] = Rc::new(UniqueDataOperator { value: 6, index: 0 });

    operators[7] = Rc::new(AddOperator { index: 0 });
    operators[8] = Rc::new(MultOperator { index: 0 });
    operators[9] = Rc::new(DivOperator { index: 0 });
    operators[10] = Rc::new(PowOperator { index: 0 });
    operators[11] = Rc::new(FactorialOperator { index: 0 });

    // 3!!/(4!*5)
    let stack = vec![
        operators[3].clone(),
        operators[11].clone(),
        operators[11].clone(),
        operators[4].clone(),
        operators[11].clone(),
        operators[5].clone(),
        operators[8].clone(),
        operators[9].clone(),
    ];

    assert_eq!(
        StackResult {
            operators: &stack.as_slice()[0..=1].to_vec(),
            value: &0,
        }
        .to_string(),
        "3!"
    );

    assert_eq!(
        StackResult {
            operators: &stack.as_slice()[0..=2].to_vec(),
            value: &0,
        }
        .to_string(),
        "3!!"
    );

    assert_eq!(
        StackResult {
            operators: &stack.as_slice()[3..=4].to_vec(),
            value: &0,
        }
        .to_string(),
        "4!"
    );

    assert_eq!(
        StackResult {
            operators: &stack.as_slice()[3..=6].to_vec(),
            value: &0,
        }
        .to_string(),
        "4!*5"
    );

    assert_eq!(
        StackResult {
            operators: &stack.as_slice()[..].to_vec(),
            value: &0,
        }
        .to_string(),
        "3!!/(4!*5)"
    );
}
