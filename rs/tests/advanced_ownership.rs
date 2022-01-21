trait Op {
    fn f(&self);
}

struct OpA;


impl Op for OpA {
    fn f(&self) {
        println!("OpA");
    }
}

struct OpB;

impl Op for OpB {
    fn f(&self) {
        println!("OpB");
    }
}

struct Container<'a> {
    all_ops: Vec<Box<dyn Op>>,
    used_ops: Vec<&'a Box<dyn Op>>, // data pointing to data in all_ops field
}

fn main() {
    let v: Vec<Box<dyn Op>> = vec![Box::new(OpA), Box::new(OpB)];

    let mut c = Container { all_ops: v, used_ops: Vec::new() };
    c.used_ops.push(&c.all_ops.get(0).unwrap());
    c.used_ops.push(&c.all_ops.get(1).unwrap());
    c.used_ops.push(&c.all_ops.get(0).unwrap());
    for op in c.used_ops {
        op.f();
    }
    c.all_ops.clear();
    // c.used.first().unwrap().f(); // cannot borrow `c.all` as mutable because it is also borrowed as immutable
}

#[test]
fn test() { main() }