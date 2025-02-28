use rand::prelude::*;

struct RandomGenerator {
    rng: ThreadRng,
}

impl RandomGenerator {
    fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }
}

impl Iterator for RandomGenerator {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rng.r#gen::<i32>())
    }
}

struct SubFold<I, B, F> {
    iter: I,
    n: usize,
    init: B,
    f: F,
}

impl<I, B, F> SubFold<I, B, F> {
    fn new(iter: I, n: usize, init: B, f: F) -> SubFold<I, B, F> {
        SubFold { iter, n, init, f }
    }
}

impl<I, B, F> Iterator for SubFold<I, B, F>
where
    I: Iterator,
    B: Clone,
    F: Fn(B, I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        // let mut acc = None;
        // for _ in 0..self.n {
        //     if let Some(val) = self.iter.next() {
        //         acc = acc
        //             .or(Some(self.init.clone()))
        //             .map(|acc| (&self.f)(acc, val));
        //     } else {
        //         break;
        //     }
        // }
        // acc

        // More functional style
        let mut subiter = self.iter.by_ref().take(self.n).peekable();
        if subiter.peek().is_some() {
            Some(subiter.fold(self.init.clone(), &self.f))
        } else {
            None
        }
    }
}

trait SubFoldable: Iterator {
    fn subfold<B, F>(self, n: usize, init: B, f: F) -> SubFold<Self, B, F>
    where
        Self: Sized,
        F: Fn(B, Self::Item) -> B,
        Self::Item: Clone,
    {
        SubFold::new(self, n, init, f)
    }
}

impl<T> SubFoldable for T where T: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_overload_of_iterator_trait() {
        let iter = RandomGenerator::new();
        SubFold::new(iter, 3, String::new(), |acc: String, x: i32| {
            acc + &x.to_string()
        })
        .next();
    }

    #[test]
    fn with_overload_of_iterator_trait() {
        // requires SubFoldable implementation

        // Implementation of this Unix command
        // </dev/urandom          |
        //     tr -dc 'a-fA-F0-9' |
        //     head -c15          |
        //     fold -w 3          |
        //     paste -sd-
        let result =
            RandomGenerator::new()
                .map(|x| (x % 256) as u8 as char)
                // .inspect(|x| println!("char: {}", x)) // show generated chars in flow 
                .filter(|x| x.is_ascii_hexdigit())
                .take(15)
                .subfold(3, String::new(), |acc, x| acc + &x.to_string())
                .fold(String::new(), |acc, s|
                    if acc.is_empty() {
                        s.to_string()
                    } else {
                        acc + "-" + &s.to_string()
                    })
            // .collect::<Vec<String>>().join("-") // Rust 1.56 stable with intermediate allocation
            // .intersperse("-".into()).collect::<String>() // Rust nightly 
            ;
        println!(">> {}", result);
    }
}
