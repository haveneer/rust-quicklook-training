#[cfg(test)]
mod tests {
    use std::iter;

    #[test]
    fn test() {
        let mut n = 0;
        let mut generator = move || {
            n = n + 1;
            let result = n;
            println!("Generate [{}]", result);
            result
        };

        iter::repeat(0)
            .map(|_| generator())
            .filter(|n| (n % 2) == 1)
            .take(1)
            .for_each(|x| println!("{}", x));
    }
}
