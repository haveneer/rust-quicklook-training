fn explicit_clone() {
    // Allocate array on heap
    let numbers = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // Move ownership to other_numbers
    let other_numbers = numbers;
    println!("{:?}", other_numbers);

    // Now we cannot access numbers anymore because value was moved.
    println!("{:?}", numbers); //~ error: borrow of moved value

    // Make a (deep) copy -> no move of ownership
    let cloned_numbers = other_numbers.clone();
    println!("clone = {:?}, source = {:?}", cloned_numbers, other_numbers);
    // Free numbers AND other_numbers vectors
}

fn main() {}