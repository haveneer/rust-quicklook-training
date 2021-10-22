use std::ops::Mul;

fn cube_by_copy<T>(x: T) -> T
    where
        T: Copy,
        T: Mul<T, Output=T>
{
    x * x * x
}

fn cube_by_ref<T>(x: &T) -> T
    where
            for<'a> T: Mul<&'a T, Output=T>,
            for<'a, 'b> &'a T: Mul<&'b T, Output=T>,
{
    x * x * x
}


fn main() {
    let x1 = cube_by_copy(2);
    let x2 = cube_by_ref(&2.0);
}

#[test]
fn test() { main() }