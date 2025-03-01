#![cfg_attr(feature = "nightly", feature(arbitrary_self_types))]

use std::pin::{pin, Pin};
use std::rc::Rc;
use std::sync::Arc;

#[cfg(feature = "nightly")]
mod details {
    #[allow(dead_code)]
    pub struct MySmartPointer<T>(Box<T>);

    impl<T> std::ops::Receiver for MySmartPointer<T> {
        type Target = T;
    }

    impl<T> MySmartPointer<T> {
        pub fn new(t: T) -> MySmartPointer<T> {
            Self(Box::new(t))
        }
    }
}

trait MyTrait {
    // Legacy receivers
    fn with_self_receiver(self);
    fn with_ref_receiver(&self);
    fn with_mut_receiver(&mut self);
    // Various other receivers since 1.33
    // and maybe more: https://rust-lang.github.io/rfcs//3519-arbitrary-self-types-v2.html
    fn with_box_receiver(self: Box<Self>);
    fn with_rc_receiver(self: Rc<Self>);
    fn with_arc_receiver(self: Arc<Self>);
    fn with_pin_receiver(self: Pin<&mut Self>);
    fn with_pin_box_receiver(self: Pin<Box<Self>>);
    fn with_funky_receiver(self: Arc<Arc<Box<Self>>>);
    // #[cfg(feature = "nightly")] // not yet even in nightly in traits
    // fn with_custom_receiver(self: details::SmartPointer<Self>);
}

struct MyType;
impl MyTrait for MyType {
    fn with_self_receiver(self) {}
    fn with_ref_receiver(&self) {}
    fn with_mut_receiver(&mut self) {}
    fn with_box_receiver(self: Box<Self>) {}
    fn with_rc_receiver(self: Rc<Self>) {}
    fn with_arc_receiver(self: Arc<Self>) {}
    fn with_pin_receiver(self: Pin<&mut Self>) {}
    fn with_pin_box_receiver(self: Pin<Box<Self>>) {}
    fn with_funky_receiver(self: Arc<Arc<Box<Self>>>) {}
}

impl MyType {
    #[cfg(feature = "nightly")]
    fn with_custom_receiver(self: details::MySmartPointer<Self>) {}
}

#[test]
fn main() {
    MyType.with_self_receiver();
    (&MyType).with_ref_receiver();
    (&mut MyType).with_mut_receiver();
    Box::new(MyType).with_box_receiver();
    Rc::new(MyType).with_rc_receiver();
    Arc::new(MyType).with_arc_receiver();
    pin!(MyType).with_pin_receiver();
    Box::pin(MyType).with_pin_box_receiver();
    Arc::new(Arc::new(Box::new(MyType))).with_funky_receiver();
    #[cfg(feature = "nightly")]
    details::MySmartPointer::new(MyType).with_custom_receiver();
}
