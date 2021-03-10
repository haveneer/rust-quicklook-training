// lazy_static allows to build complex structures as static data
use lazy_static::*;

struct ComplexStructure {
    field: String,
    all_want_you_want: std::collections::HashMap<u32, String>,
}

impl ComplexStructure {
    pub fn new() -> Self {
        Self {
            field: "Hello".to_owned(),
            all_want_you_want: vec![
                (1, "A".to_owned()),
                (2, "B".to_owned()),
                (3, "C".to_owned()),
                (4, "D".to_owned()),
            ]
            .into_iter()
            .collect(),
        }
    }
}

lazy_static! {
    static ref COMPLEX_STRUCTURE: ComplexStructure = {
        // Arbitrary complex initialization code
        ComplexStructure::new()
    };
}

#[test]
fn test() {
    println!("{}", COMPLEX_STRUCTURE.field);
    COMPLEX_STRUCTURE
        .all_want_you_want
        .iter()
        .for_each(|(k, v)| println!("{} : {}", k, v));
}
