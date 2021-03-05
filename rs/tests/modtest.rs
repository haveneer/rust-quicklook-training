pub mod modmodtest {
    fn add_n(n: i32) -> impl Fn(i32) -> i32 {
        move |x| n + x
    }

    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }


// What is
// * 'a   : is lifetime
//        * https://stackoverflow.com/questions/22048673/what-are-the-rust-types-denoted-with-a-single-apostrophe
//        * https://stackoverflow.com/questions/17490716/lifetimes-in-rust
// * impl : https://doc.rust-lang.org/std/keyword.impl.html
// * Fn   : https://doc.rust-lang.org/std/ops/trait.Fn.html

    fn compose<'a, F, G, T, U, V>(f: F, g: G) -> impl Fn(T) -> V + 'a
        where F: Fn(U) -> V + 'a,
              G: Fn(T) -> U + 'a
    {
        move |x| f(g(x))
    }

// Not OK
//fn compose3<'a,F,G,H,T,U,V>(f: F, g: G) -> H
//    where F: Fn(U) -> V + 'a,
//          G: Fn(T) -> U + 'a,
//          H: Fn(T) -> V + 'a,
//{
//    move |x| f(g(x))
//}


    fn compose2<A, B, C>(f: impl Fn(B) -> C,
                         g: impl Fn(A) -> B)
                         -> impl Fn(A) -> C
    {
        move |x| f(g(x))
    }


// Read them
// Pointers: http://rust-class.org/class-9-pointers-in-rust.html
// Rust by example : https://doc.rust-lang.org/rust-by-example/index.html


    fn add_3() -> impl Fn(i32) -> i32 {
        move |x| 3 + x
    }

    #[test]
    fn test() {
        let _x = compose(add_n(3), add_n(5));

        // https://doc.rust-lang.org/error-index.html#E0562
        // let t : impl Fn<(i32), Output=i32> = compose(add_n(3), add_n(5));

        let f = add_3;
        println!("The answer to life is {}.", f()(3));


        let f2 = compose(add_n(3), add_n(5));
        println!("The answer to life is {}.", f2(0));

        let f3 = compose2(add_n(3), add_n(5));
        println!("The answer to life is {}.", f3(0));

        let g = multiply;
        println!("The answer to life is {}.", g(3, 4));


        let adder = add_n(40);
        println!("The answer to life is {}.", adder(2));
    }
}