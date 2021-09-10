trait Trait {
    fn foo(&self);
}

impl Trait for i64 {
    fn foo(&self) {}
}

// # Argument position : equivalent forms (no deprecation)
fn impl_in2015<T: Trait>(arg: T) {}

// turbo-fish allowed: impl_in2015::<i64>()
fn impl_in2018(arg: impl Trait) {}

// # Return position (no deprecation; not same perf)
fn impl_out2015() -> Box<dyn Trait> {
    // requires heap allocation
    Box::new(0_i64)
}

fn impl_out2018() -> impl Trait {
    // does not require heap allocation
    0_i64
}

fn returns_closure2015() -> Box<dyn Fn(i64) -> i64> {
    Box::new(|x| x + 1)
}

fn returns_closure2018() -> impl Fn(i64) -> i64 {
    |x| x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dyn_impl_demo() {
        impl_in2015(0_i64);
        impl_in2018(0_i64);

        let x = impl_out2015();
        x.foo();

        let x = impl_out2018();
        x.foo();

        let f = returns_closure2015();
        let y = f(0);

        let f = returns_closure2018();
        let y = f(0);
    }

    #[test]
    fn size_of_vec_of_dyn() {
        let v_dyn: [&dyn Trait; 4] = [&1i64, &2, &3, &4];
        println!("size_of(v_dyn) = {}", std::mem::size_of_val(&v_dyn));
        for &i in v_dyn.iter() {
            i.foo();
        }
    }
    
    #[test]
    fn size_of_vec_of_impl() {
        let v_impl: [i64; 4] = [1, 2, 3, 4];
        println!("size_of(v_impl) = {}", std::mem::size_of_val(&v_impl));
        for &i in v_impl.iter() {
            i.foo();
        }
    }
}
