use clap::Parser;
use best_count::*;

#[derive(Parser, Debug)]
struct Args {
    /// Number of times to greet
    #[clap(short, long, default_value_t = 2022)]
    target: u64,
}

fn main() {
    let args = Args::parse();

    println!("Target = {}", args.target);

    let mut operators = Vec::<Box<dyn Operator>>::new();
    // Pour utiliser les nombres dans un ordre quelconque
    let mut maxdigit = 0;
    for i in 1..=5 {
        operators.push(Box::new(UniqueDataOperator { value: i, index: operators.len() }));
        maxdigit += 1;
    }

    // let mut prev: Option<&dyn Operator> = None;
    // for i in 1..=8 {
    //     operators.push(Box::new(SeqDataOperator { value: i, prev, index: operators.len() }));
    //     prev = operators
    //         .last()
    //         .map(|x| x.as_ref());
    // }
    operators.push(Box::new(AddOperator { index: operators.len() }));
    operators.push(Box::new(MultOperator { index: operators.len() }));
    operators.push(Box::new(DivOperator { index: operators.len() }));
    operators.push(Box::new(PowOperator { index: operators.len() }));
    operators.push(Box::new(FactorialOperator { index: operators.len() }));

    let test = {
        // let op = prev.unwrap();
        move |s: &Stack| {
            // s.is_used(op.borrow())
            maxdigit == s.data_count()
        }
    };
    let results = compute(args.target, operators, test);

    if results.is_empty() {
        println!("No solution has been found");
    } else {
        //     println!("Found solution(s):");
        //     results.into_iter().for_each(|s| println!("{}", s));
    }
}