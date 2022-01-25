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
    all_ops: Vec<&'a dyn Op>,
    used_ops: Vec<&'a dyn Op>, // data pointing to data in all_ops field
}

fn main() {
    let v: Vec<&dyn Op> = vec![&OpA, &OpB];

    let mut c = Container { all_ops: v, used_ops: Vec::new() };
    c.used_ops.push(*c.all_ops.get(0).unwrap());
    c.used_ops.push(*c.all_ops.get(1).unwrap());
    c.used_ops.push(*c.all_ops.get(0).unwrap());
    for op in c.used_ops.iter() {
        op.f();
    }
    let c2 = c; // cannot move out of `c` because it is borrowed
}

#[test]
fn test() { main() }


