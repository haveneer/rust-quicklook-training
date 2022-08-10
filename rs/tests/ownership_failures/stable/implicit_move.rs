mod tests {
    pub fn implicit_move() {
        let numbers = vec![1, 2, 3, 4, 5];

        let other_numbers = numbers;

        println!("{:?}", other_numbers);
        println!("{:?}", numbers); //~ error: borrow of moved value: `numbers`
    }
}

fn main() {}