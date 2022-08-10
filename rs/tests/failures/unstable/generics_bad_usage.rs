fn foo<T>(arg: T) -> T { return arg + 1; }

fn main() {
    foo(1);
    foo(3.14);
    foo("Hello");
}

// Solution au moins pour foo(1)
// fn foo<T: std::ops::Add<i32, Output=T>>(arg: T) -> T { return arg + 1; }
