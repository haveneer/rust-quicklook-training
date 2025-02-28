use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

pub fn coroutine_test() {
    let mut generator = #[coroutine]
    || {
        let xs = vec![1, 2, 3];
        let mut sum = 0;
        for x in xs {
            sum += x;
            yield sum;
        }
        "The end"
    };

    assert_eq!(
        Pin::new(&mut generator).resume(()),
        CoroutineState::Yielded(1)
    );
    assert_eq!(
        Pin::new(&mut generator).resume(()),
        CoroutineState::Yielded(3)
    );
    assert_eq!(
        Pin::new(&mut generator).resume(()),
        CoroutineState::Yielded(6)
    );
    assert_eq!(
        Pin::new(&mut generator).resume(()),
        CoroutineState::Complete("The end")
    );
}
