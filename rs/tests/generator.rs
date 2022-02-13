#![feature(generators)]
#![feature(generator_trait)]
// cf https://doc.rust-lang.org/beta/unstable-book/language-features/generators.html

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

fn main() {
    let mut generator = || {
        let xs = vec![1, 2, 3];
        let mut sum = 0;
        for x in xs {
            sum += x;
            yield sum;
        }
        "The end"
    };

    assert_eq!(Pin::new(&mut generator).resume(()), GeneratorState::Yielded(1));
    assert_eq!(Pin::new(&mut generator).resume(()), GeneratorState::Yielded(3));
    assert_eq!(Pin::new(&mut generator).resume(()), GeneratorState::Yielded(6));
    assert_eq!(Pin::new(&mut generator).resume(()), GeneratorState::Complete("The end"));
}

#[test]
fn test() { main() }