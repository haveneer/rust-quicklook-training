trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &'static str;
}

struct Square {
    side: f64,
}

struct Circle {
    radius: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn name(&self) -> &'static str {
        "Square"
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
    fn name(&self) -> &'static str {
        "Circle"
    }
}

fn display_area(s: &impl Shape) -> String {
    format!("{} -> {}", s.name(), s.area())
}

fn main() {
    let v_impl: [Square; 2] = [Square { side: 2. }, Square { side: 1. }];
    println!("size_of(v_impl) = {}", std::mem::size_of_val(&v_impl));

    // Static dispatch
    let square = Square { side: 2. };
    println!("{}", display_area(&square));

    let circle = Circle { radius: 2. };
    println!("{}", display_area(&circle));
}

#[test]
fn test() {
    main()
}
