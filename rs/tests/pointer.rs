static GLOBAL: i32 = 1000; // global static

fn noop() -> *const i32 {
    let noop_local = 12345; // local variable, may be clear this away
    &noop_local as *const i32 // return the address as a raw pointer
}

#[test]
fn main() {
    let local_str = "a";
    let local_int = 123;
    let boxed_str = Box::new('b');
    let boxed_int = Box::new(789);
    let fn_int = noop();

    println!("GLOBAL:    {:p}", &GLOBAL as *const i32);
    println!("local_str: {:p}", local_str as *const str);
    println!("local_int: {:p}", &local_int as *const _); // using _ as type will place the right type
    println!("boxed_int: {:p}", Box::into_raw(boxed_int));
    println!("boxed_str: {:p}", Box::into_raw(boxed_str));
    println!("fn_int:    {:p}", fn_int);
}
