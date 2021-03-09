extern crate procedural_macro; // cf produral_macro local crate
use procedural_macro::make_answer;
use procedural_macro::CountFields;

make_answer!(); // create an answer function

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        println!("Answer:{}", answer1());
    }

    #[derive(CountFields)]
    enum Any {
        A,
        _B,
        _C,
    }

    #[test]
    fn test2() {
        let a = Any::A {};

        println!("Field found:{}", a.count_field());
    }
}
