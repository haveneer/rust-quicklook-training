use enum_dispatch::enum_dispatch;

#[enum_dispatch]
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &'static str;
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn name(&self) -> &'static str {
        "Square"
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
    fn name(&self) -> &'static str {
        "Circle"
    }
}

#[enum_dispatch(Shape)]
enum ShapeEnum {
    Square(Square),
    Circle(Circle),
}

fn display_area(s: &ShapeEnum) -> String {
    format!("{} -> {}", s.name(), s.area())
}

fn main() {
    let v_enum: [ShapeEnum; 2] = [Square { side: 2.0 }.into(), Circle { radius: 2.0 }.into()];
    println!("size_of(v_impl) = {}", std::mem::size_of_val(&v_enum));

    for s in v_enum.iter() {
        println!("{}", display_area(s));
    }
}

#[test]
fn test() {
    main();
}
