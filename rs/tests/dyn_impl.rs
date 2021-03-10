trait Trait {
    fn foo(&self);
}

impl Trait for i32 {
    fn foo(&self) {}
}

// # Argument position : equivalent forms (no deprecation)
fn impl_in2015<T: Trait>(arg: T) {} // turbo-fish allowed: impl_in2015::<i32>()
fn impl_in2018(arg: impl Trait) {}

// # Return position (no deprecation; not same perf)
fn impl_out2015() -> Box<dyn Trait> {
    // requires heap allocation
    Box::new(0_i32)
}

fn impl_out2018() -> impl Trait {
    // does not require heap allocation
    0_i32
}

fn returns_closure2015() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_closure2018() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        impl_in2015(0_i32);
        impl_in2018(0_i32);

        let x = impl_out2015();
        x.foo();

        let x = impl_out2018();
        x.foo();
        
        let f = returns_closure2015();
        let y = f(0);

        let f = returns_closure2018();
        let y = f(0);
    }
}
