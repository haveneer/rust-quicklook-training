use std::cell::{Cell, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

#[test]
fn test_cell() {
    // single threaded / Copy data
    // like side gets the idea, but it is illegal Rust code
    let x = Cell::new(1); //    like let mut x = 1;
    let y = &x; //              like let y = &mut x;
    let z = &x; //              like let z = &mut x;
    x.set(2); //                like x = 2;
    y.set(3); //                like *y = 3;
    z.set(4); //                like *z = 4;
    println!("{}", x.get()); // like println!("{}", x;
}

#[test]
fn test_refcell1() {
    let x = RefCell::new(vec![1, 2, 3, 4]);
    {
        println!("{:?}", *x.borrow())
    }

    {
        let mut my_ref = x.borrow_mut();
        // let mut my_ref2 = x.borrow_mut(); // multiple mut borrow will panic
        my_ref.push(1);
    }

    {
        let my_ref = x.borrow(); 
        println!("{:?}", *my_ref);
    }
}

#[test]
fn test_refcell2() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    // Create a new block to limit the scope of the dynamic borrow
    {
        let mut map: RefMut<_> = shared_map.borrow_mut();
        map.insert("africa", 92388);
        map.insert("kyoto", 11837);
        map.insert("piccadilly", 11826);
        map.insert("marbles", 38);
    }

    // Note that if we had not let the previous borrow of the cache fall out
    // of scope then the subsequent borrow would cause a dynamic thread panic.
    // This is the major hazard of using `RefCell`.
    let total: i32 = shared_map.borrow().values().sum();
    println!("{}", total);
}
