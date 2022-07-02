// edition:2018

use futures::executor::block_on;
use std::time::Duration;

async fn async_main() {
    let mut numbers = vec![1; 100];
    let sum_future = sum(&numbers);
    std::thread::sleep(Duration::from_millis(100));
    add(&mut numbers); //~ cannot borrow `numbers` as mutable
    println!("The sum is {}", sum_future.await);
}

fn add(numbers: &mut Vec<i32>) {
    numbers.push(42);
}

async fn sum(numbers: &Vec<i32>) -> i32 {
    let iter = numbers.iter();
    std::thread::sleep(Duration::from_millis(200));
    iter.sum()
}

fn main() {
    block_on(async_main());
}
