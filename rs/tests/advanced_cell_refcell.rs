use std::cell::{Cell, RefCell};

#[test]
// Copy Data
fn test_cell() {
    let x = Cell::new(1); // let mut x = 1;
    let y = &x; //                let y = &mut x;
    let z = &x; //                let z = &mut x;
    x.set(2); //                         *x = 2;
    y.set(3); //                         *y = 3;
    assert_eq!(z.replace(4), 3); //      *z = 4;
    println!("{}", x.get()); //              println!("{}", x);
}

#[test]
// Non-Copy Data
fn test_refcell() {
    let x = RefCell::new(vec![1, 2, 3, 4]);
    {
        let mut my_mut_ref = x.borrow_mut();
        // let mut my_ref2 = x.borrow_mut(); // multiple mutable borrows will panic
        my_mut_ref.push(5);
    }
    {
        // TODO What if we remove the surrounding blocks...
        let my_ref = x.borrow();
        println!("{:?}", *my_ref);
    }
}

#[test]
// Equivalent with compile-time control
fn test_no_refcell() {
    let mut x = vec![1, 2, 3, 4];
    let my_ref = &mut x;
    my_ref.push(5);
    let my_ref = &x;
    println!("{:?}", *my_ref);
}
