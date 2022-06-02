#![allow(unused_variables)]
use std::ops::Deref;

#[test]
fn default_deref() {
    let x: i32 = 1;
    let y = &x;
    let z: *const i32 = y; // legal but unusual (don't use direct pointer)

    // if x == y { } // error: compare i32 with &i32
    if x == *y {} // OK: compare values
    if &x == y {} // OK: compare addresses
    if z == y {} // OK: compare pointers

    // Use deref operator * to get access to a value behind a reference (or a pointer)
}

#[test]
fn custom_deref() {
    let table = vec![1, 1, 2, 3, 5, 8, 13, 21];
    let mut myref = MyRef { index: 0, table };

    assert_eq!(*myref, 1);
    myref.move_next();
    assert_eq!(*myref, 1);
    myref.move_next();
    assert_eq!(*myref, 2);
    // ...

    // Not *x and x.deref() are not exactly the same thing
    // If T implements Deref<Target = U>, and x is a value of type T, then:
    // In immutable contexts, *x on non-pointer types is equivalent to *Deref::deref(&x).
    let z: i32 = *myref;
    let z: &i32 = myref.deref();

    // More on Deref coercion
    // Values of type &T are coerced to values of type &U
    // T implicitly implements all the (immutable) methods of the type U.
    let _ = myref.abs(); // .abs() is originally a function on i32 !!!

    // basic types already implement Deref trait
    let x: i32 = 1; // x.abs() is legal
    let y = &x; // y.abs() is also legal

    let z: i32 = *y;
    let z: &i32 = y.deref();
}

struct MyRef {
    index: usize,
    table: Vec<i32>,
}

// There also exists DerefMut for mutable reference
impl Deref for MyRef {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        // always returns a reference !
        // it could be dereferenced later using operator *
        &self.table[self.index]
    }
}

impl MyRef {
    fn move_next(&mut self) {
        self.index += 1;
    }
}
