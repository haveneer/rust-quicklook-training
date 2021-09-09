fn borrow_and_functions() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    println!("The sum is {}",    // Passes reference, 
             consume(&numbers)); // keeps ownership
    println!("The sum is {}",                // Mutable reference,
             add_and_consume(&mut numbers)); // keeps ownership

    println!("{:?}", numbers);
}

fn consume(numbers: &Vec<i32>) -> i32 {
    // numbers is READ-ONLY, cannot be mutated
    numbers.push(42);  //~ error: cannot borrow `*numbers` as mutable
    let sum: i32 = numbers.iter().sum();
    sum
}

fn add_and_consume(numbers: &mut Vec<i32>) -> i32 {
    numbers.push(42);
    consume(numbers)
}

fn main() {}