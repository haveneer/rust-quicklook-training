use std::cell::{Cell, RefCell};
use std::collections::HashMap;

/// A `Fibonacci` calculator with an internal memoization cache.
/// It also tracks how many times we had a cache hit or cache miss
pub struct Fibonacci {
    cache: RefCell<HashMap<u64, u64>>,
    hits: Cell<u64>,
    misses: Cell<u64>,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci {
            cache: RefCell::new(HashMap::new()),
            hits: Cell::new(0),
            misses: Cell::new(0),
        }
    }

    pub fn compute(&self, n: u64) -> u64 {
        let cache_borrow = self.cache.borrow();
        if let Some(&cached_value) = cache_borrow.get(&n) {
            self.hits.set(self.hits.get() + 1); // cache hit count
            return cached_value;
        }
        drop(cache_borrow); // reduce cache_borrow lifetime
        self.misses.set(self.misses.get() + 1); // cache miss count

        let result = if n < 2 {
            n // base cases
        } else {
            self.compute(n - 1) + self.compute(n - 2)
        };

        self.cache.borrow_mut().insert(n, result); // Store the computed value in the cache.

        result
    }

    pub fn cache_hits(&self) -> u64 {
        self.hits.get()
    }

    pub fn cache_misses(&self) -> u64 {
        self.misses.get()
    }
}

fn main() {
    let fib = Fibonacci::new();

    assert_eq!(fib.compute(10), 55);
    {
        // details
        println!(
            "Cache hits/misses: {}/{}",
            fib.cache_hits(),
            fib.cache_misses()
        );

        assert_eq!(fib.compute(10), 55); // Should be in the cache
        println!(
            "Cache hits/misses: {}/{}",
            fib.cache_hits(),
            fib.cache_misses()
        );
    }
}

#[test]
fn test() {
    main()
}
