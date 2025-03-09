use std::marker::PhantomPinned;
use std::pin::pin;

struct MyStruct {
    value: i32,
    _pin: PhantomPinned, // Make it `!Unpin`
}

impl MyStruct {
    fn set_value(&mut self, new_value: i32) {
        self.value = new_value;
    }
}

fn main() {
    let pinned: Pin<&mut MyStruct> = pin!(MyStruct {
        value: 10,
        _pin: PhantomPinned
    });

    pinned.set_value(0);

    // let mut other_value = MyStruct { value: 3, _pin: PhantomPinned };
    // std::mem::swap(&mut other_value, pinned.get_mut());
}
