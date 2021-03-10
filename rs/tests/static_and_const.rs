// definitions out of any body
const GLOBAL_CONST_STRING: &str = "Hello"; // const are inlined and don't have fixed memory address
const GLOBAL_CONST_VALUE: usize = 65536;

static GLOBAL_STATIC_STRING: &str = "Hello"; // static are not inlined and have a fixed memory address
static mut GLOBAL_STATIC_VALUE: usize = 65536; // and could be mut(able)

// Differences static vs const
// * https://doc.rust-lang.org/1.30.0/book/first-edition/const-and-static.html
// * https://doc.rust-lang.org/reference/items/static-items.html

struct Data {
    field: [u8; Self::SIZE],
}

impl Data {
    const SIZE: usize = 64;

    pub fn foo(&self) {
        assert_eq!(self.field.len(), Self::SIZE);
    }
}

#[test]
fn main() {
    const LOCAL_CONST_STRING: &str = "Hello"; // same comments as global scope
    const LOCAL_CONST_VALUE: usize = 65536;
    static LOCAL_STATIC_STRING: &str = "Hello";
    static mut LOCAL_STATIC_VALUE: usize = 65536;

    unsafe { GLOBAL_STATIC_VALUE += 1 };
    unsafe { LOCAL_STATIC_VALUE += 1 };

    assert_eq!(
        unsafe { GLOBAL_STATIC_VALUE }, // /!\ static variable are always unsafe
        unsafe { LOCAL_STATIC_VALUE }
    );

    println!("{:p}", &GLOBAL_CONST_VALUE);
    println!("{:p}", &LOCAL_CONST_VALUE);

    println!("{:p}", unsafe {
        // optional raw pointer conversion {:p} will do it
        &GLOBAL_STATIC_VALUE /* as *const _ */
    });
    println!("{:p}", unsafe {
        &LOCAL_STATIC_VALUE /* as *const _ */
    });

    let data = Data { field: [0u8; 64] };
    data.foo();
}
