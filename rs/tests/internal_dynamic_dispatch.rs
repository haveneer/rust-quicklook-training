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

fn display_area(s: &dyn Shape) -> String {
    format!("{} -> {}", s.name(), s.area())
}

fn main() {
    let v_dyn: [&dyn Shape; 2] = [&Square { side: 2. }, &Circle { radius: 2. }];
    println!("size_of(v_dyn) = {}", std::mem::size_of_val(&v_dyn));

    // Dynamic dispatch
    for &s in v_dyn.iter() {
        println!("{}", display_area(s));
    }
}

#[test]
fn test() { main() }