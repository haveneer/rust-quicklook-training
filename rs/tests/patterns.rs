#[test]
fn trivial_match_enum() {
    enum Tree {
        Empty,
        Node { left: Box<Tree>, right: Box<Tree> },
    }

    let tree = {
        Tree::Node {
            left: Box::new(Tree::Empty),
            right: Box::new(Tree::Node {
                left: Box::new(Tree::Empty),
                right: Box::new(Tree::Empty),
            })}
        };

        match tree {
            Tree::Empty => { println!("Empty Tree"); }
            Tree::Node { left: _, right: _ } => { println!("Tree with nodes"); }
        }
    }

    #[test]
    fn match_enum() {
        // strum requires the Default's trait on subfield (here u8)
        #[derive(Debug, strum::EnumIter)]
        enum MyEnum {
            A,
            B,
            C(u8),
        }
        ;

        use strum::IntoEnumIterator;
        for e in MyEnum::iter() {
            match e {
                MyEnum::A => println!("A = {:?}", e),
                MyEnum::B => println!("B = {:?}", e),
                MyEnum::C(x) => println!("C{} = {:?}", x, e),
                // all case required or not compile
            }
        }

        println!("{:-<20}", "");

        for e in MyEnum::iter() {
            match e {
                MyEnum::A => println!("A = {:?}", e),
                MyEnum::B => println!("B = {:?}", e),
                _ => println!("Other = {:?}", e), // catch all other cases
            }
        }

        println!("{:-<20}", "");

        for e in MyEnum::iter() {
            match e {
                A => println!("/!\\ A = {:?}", A),
                B => println!("/!\\ B = {:?}", B),
                _ => println!("/!\\ Other = {:?}", e), // catch all other cases
            }
        }

        println!("{:-<20}", "");

        for e in MyEnum::iter() {
            match e {
                MyEnum::A => println!("A"),
                b @ MyEnum::B => println!("B = {:?}", b),
                c @ MyEnum::C(0) => println!("C(0) = {:?}", c),
                MyEnum::C(x) if x == 1 => println!("C(x=1) = {:?}", e),
                // c @ MyEnum::C(x) => println!("C(x=1) = {:?}", c), // mixing binding and @ is unstable (https://github.com/rust-lang/rust/issues/65490)
                x @ _ => println!("Other = {:?}", x),
                // x @ _ => println!("/!\\ Other = {:?}", e), // cannot use borrowed e -> x
            }
        }
    }

    #[test]
    fn match_struct() {
        #[derive(Debug)]
        struct MyStruct {
            a: bool,
            b: bool,
        }

        let mut data = [
            MyStruct { a: true, b: false },
            MyStruct { a: false, b: false },
            MyStruct { a: true, b: true },
            MyStruct { a: false, b: true },
        ];

        for s in data.iter() {
            match s {
                MyStruct { a: x, b: true } => println!("Case 1: {:?}", s),
                // clever match: can see b: false is missing
                MyStruct { a: true, b: false } => println!("Case 2: {:?}", s),
                x @ _ => println!("Other case : {:?}", x),
            }
        }

        println!("{:-<20}", "");

        for s in data.iter_mut() {
            match s {
                MyStruct { a: x, b: true } => *x = true,
                MyStruct { b: false, .. } => {} // only b field is useful; the others don't matter
            }
        }
        for s in data.iter() {
            println!("{:?}", s);
        }
    }

    #[test]
    fn pattern_multiple() {
        fn num_to_ordinal(x: u32) -> String {
            let suffix = match (x % 10, x % 100) {
                (1, 1) | (1, 21..=91) => "st",
                (2, 2) | (2, 22..=92) => "nd",
                (3, 3) | (3, 23..=93) => "rd",
                _ => "th",
            };
            return format!("{}{}", x, suffix);
        }

        assert_eq!(num_to_ordinal(0), "0th");
        assert_eq!(num_to_ordinal(1), "1st");
        assert_eq!(num_to_ordinal(12), "12th");
        assert_eq!(num_to_ordinal(22), "22nd");
        assert_eq!(num_to_ordinal(43), "43rd");
        assert_eq!(num_to_ordinal(67), "67th");
        assert_eq!(num_to_ordinal(1901), "1901st");
    }

    #[test]
    fn math_rules() {
        for i in 0..=11u8 {
            let annotation = match i {
                0 => "Null",
                1...4 => "Low", // ... range are deprecated prefer ..=
                5..=7 => "Ok",
                8..=9 => "Very good",
                10 => "Perfect",
                _ => "Overrated!",
                // num if num > 20 => "Overrated!", // not yet ok: error: pattern `11_u8..=u8::MAX` not covered
            };
            println!("{} => {}", i, annotation);
        }
    }

    #[test]
    fn more_patterns() {
        // is coming in edition 2021
        // https://github.com/rust-lang/rust/issues/54883
    }
