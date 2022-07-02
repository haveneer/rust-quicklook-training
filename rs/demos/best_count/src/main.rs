use best_count::*;
use clap::Parser;
use std::borrow::Borrow;
use std::rc::Rc;

#[derive(Parser, Debug)]
struct Args {
    /// Number of times to greet
    #[clap(short, long, default_value_t = 2022)]
    target: u64,
}

fn main() {
    let args = Args::parse();

    println!("Target = {}", args.target);

    let mut operators = Vec::<Rc<dyn Operator>>::new();
    // operators.push(Rc::new(UniqueDataOperator { value: 1, index: operators.len() }));
    // operators.push(Rc::new(UniqueDataOperator { value: 2, index: operators.len() }));
    // operators.push(Rc::new(UniqueDataOperator { value: 3, index: operators.len() }));

    let mut prev: Option<Rc<dyn Operator>> = None;
    for i in 1..=8 {
        let next = Rc::new(SeqDataOperator {
            value: i,
            prev: prev.map(|o| o.clone()),
            index: operators.len(),
        });
        operators.push(next.clone());
        prev = Some(next);
    }
    operators.push(Rc::new(AddOperator {
        index: operators.len(),
    }));
    operators.push(Rc::new(MultOperator {
        index: operators.len(),
    }));
    operators.push(Rc::new(DivOperator {
        index: operators.len(),
    }));
    operators.push(Rc::new(PowOperator {
        index: operators.len(),
    }));
    operators.push(Rc::new(FactorialOperator {
        index: operators.len(),
    }));

    let test = {
        let op = prev.unwrap();
        move |s: &Stack| s.is_used(op.borrow())
    };
    let results = compute(args.target, operators, test);

    if results.is_empty() {
        println!("No solution has been found");
    } else {
        //     println!("Found solution(s):");
        //     results.into_iter().for_each(|s| println!("{}", s));
    }
}
