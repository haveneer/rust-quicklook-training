// More info :
// * https://medium.com/nearprotocol/understanding-rust-lifetimes-e813bcd405fa
// * https://doc.rust-lang.org/1.9.0/book/lifetimes.html

fn max1(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

#[test]
fn lifetime1() {
    let r: i32; // copy value: no lifetime issue
    let a: i32 = { 2 + 2 };
    {
        let b: i32;
        {
            b = 5;
            r = max1(a, b);
            println!("max is {:?}", r);
        }
        println!("b is {:?}", b);
        println!("max is {:?}", r);
    }
    println!("a is {:?}", a);
    println!("max is {:?}", r);
}

fn max2<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    println!("x is {:?}", *x);
    println!("y is {:?}", *y);
    if *x > *y {
        x
    } else {
        y
    }
}

#[test]
fn lifetime2() {
    // On usage of reference:
    // https://stackoverflow.com/questions/36335342/meaning-of-the-ampersand-and-star-symbols-in-rust

    let r: &i32;
    let a: i32 = 4;
    {
        let b: i32 = 5;
        {
            r = max2(&a, &b);
            println!("max is {:?}", r);
        }
        println!("b is {:?}", b);
    }
    println!("a is {:?}", a);
    //    println!("max is {:?}", r.v); // too short lifetime if b
}

struct W {
    v: i32,
}

fn max3<'a>(x: &'a W, y: &'a W) -> &'a W {
    println!("x is {:?}", x.v);
    println!("y is {:?}", y.v);
    if x.v > y.v {
        x
    } else {
        y
    }
}

#[test]
fn lifetime3() {
    let r: &W;
    let a: W = W { v: 4 };
    {
        let b: W;
        {
            b = W { v: 5 };
            r = max3(&a, &b);
            println!("max is {:?}", r.v);
        }
        println!("b is {:?}", b.v);
        println!("max is {:?}", r.v);
    }
    println!("a is {:?}", a.v);
    //    println!("max is {:?}", r.v); // too short lifetime if b
}

#[test]
fn lifetime4() {
    // Good idea but unstable : https://github.com/rust-lang/rust/issues/48594
    //    // This is subtyping
    //    let val1 = 42;
    //    let val2 = 24;
    //    'x: {
    //        let ref1 : &'x i32 = val1;
    //        'y: {
    //            let mut ref2 : &'y mut i32= val2;
    //            ref2 = ref1;
    //        }
    //    }
}

// 'a : 'b : 'a outlives 'b <=> 'a lasts at least as long as 'b
fn f<'a, 'b>(x: &'a i32, mut y: &'b i32)
where
    'a: 'b,
{
    y = x; // &'a i32 is a subtype of &'b i32 because 'a: 'b
    let r: &'b &'a i32 = &&0; // &'b &'a i32 is well formed because 'a: 'b
}

// See https://doc.rust-lang.org/reference/trait-bounds.html
// https://doc.rust-lang.org/reference/subtyping.html

#[test]
fn lifetime5() {
    let x = 1;
    {
        let y = 2;
        let z = f(&x, &y);
        drop(y);
        f(&y, &x);
    };
}

#[test]
#[rustfmt::skip]
fn lifetime_failures() {
    let t = trybuild::TestCases::new();

    // let version_path = if cfg!(feature = "nightly") { "unstable" } else { "stable" };

    t.compile_fail("tests/failures/bad_lifetime.rs");
}