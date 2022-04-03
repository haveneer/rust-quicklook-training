use best_count::*;
pub use best_count::stack::StackResult;

#[test]
fn expression_display() {
    let operators: Vec<Box<dyn Operator>> =
        vec![
            /* 0 */ Box::new(UniqueDataOperator { value: 0, index: 0 }),
            /* 1 */ Box::new(UniqueDataOperator { value: 1, index: 0 }),
            /* 2 */ Box::new(UniqueDataOperator { value: 2, index: 0 }),
            /* 3 */ Box::new(UniqueDataOperator { value: 3, index: 0 }),
            /* 4 */ Box::new(UniqueDataOperator { value: 4, index: 0 }),
            /* 5 */ Box::new(UniqueDataOperator { value: 5, index: 0 }),
            /* 6 */ Box::new(UniqueDataOperator { value: 6, index: 0 }),
            /* 7  */ Box::new(AddOperator { index: 0 }),
            /* 8  */ Box::new(MultOperator { index: 0 }),
            /* 9  */ Box::new(DivOperator { index: 0 }),
            /* 10 */  Box::new(PowOperator { index: 0 }),
            /* 11 */  Box::new(FactorialOperator { index: 0 }),
        ];

    // 3!!/(4!*5)
    let stack =
        vec![&operators[3],
             &operators[11],
             &operators[11],
             &operators[4],
             &operators[11],
             &operators[5],
             &operators[8],
             &operators[9],
        ];

    // assert_eq!(StackResult {
    //     operators: &stack.as_slice()[0..=1].to_vec(),
    //     value: &0,
    // }.to_string(), "3!");
    // 
    // assert_eq!(StackResult {
    //     operators: &stack.as_slice()[0..=2].to_vec(),
    //     value: &0,
    // }.to_string(), "3!!");
    // 
    // assert_eq!(StackResult {
    //     operators: &stack.as_slice()[3..=4].to_vec(),
    //     value: &0,
    // }.to_string(), "4!");
    // 
    // assert_eq!(StackResult {
    //     operators: &stack.as_slice()[3..=6].to_vec(),
    //     value: &0,
    // }.to_string(), "4!*5");
    // 
    // assert_eq!(StackResult {
    //     operators: &stack.as_slice()[..].to_vec(),
    //     value: &0,
    // }.to_string(), "3!!/(4!*5)");
}









