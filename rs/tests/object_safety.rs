#[test]
#[rustfmt::skip]
fn object_safety_failures() {
    let t = trybuild::TestCases::new();

    let version_path = if cfg!(feature = "nightly") { "unstable" } else { "stable" };

    t.compile_fail(format!("tests/failures/{version_path}/object_safety.rs"));
}

use std::pin::{pin, Pin};
use std::rc::Rc;
use std::sync::Arc;

trait MyTrait {
    // Legacy receivers
    fn with_ref_receiver(&self);
    fn with_mut_receiver(&mut self);
    // Various other receivers since 1.33
    fn with_box_receiver(self: Box<Self>);
    fn with_rc_receiver(self: Rc<Self>);
    fn with_arc_receiver(self: Arc<Self>);
    fn with_pin_receiver(self: Pin<&mut Self>);
    fn with_pin_box_receiver(self: Pin<Box<Self>>);
    fn with_pin_arc_receiver(self: Pin<Arc<Self>>);
}

struct MyType;
impl MyTrait for MyType {
    fn with_ref_receiver(&self) {}
    fn with_mut_receiver(&mut self) {}
    fn with_box_receiver(self: Box<Self>) {}
    fn with_rc_receiver(self: Rc<Self>) {}
    fn with_arc_receiver(self: Arc<Self>) {}
    fn with_pin_receiver(self: Pin<&mut Self>) {}
    fn with_pin_box_receiver(self: Pin<Box<Self>>) {}
    fn with_pin_arc_receiver(self: Pin<Arc<Self>>) {}
}

#[test]
fn main() {
    let x: &dyn MyTrait = &mut MyType;
    x.with_ref_receiver();

    let x: &mut dyn MyTrait = &mut MyType;
    x.with_mut_receiver();

    let x: Box<dyn MyTrait> = Box::new(MyType);
    x.with_box_receiver();

    let x: Rc<dyn MyTrait> = Rc::new(MyType);
    x.with_rc_receiver();

    let x: Arc<dyn MyTrait> = Arc::new(MyType);
    x.with_arc_receiver();

    let x: Pin<&mut dyn MyTrait> = pin!(MyType);
    x.with_pin_receiver();

    let x: Pin<Box<dyn MyTrait>> = Box::pin(MyType);
    x.with_pin_box_receiver();

    let x: Pin<Arc<dyn MyTrait>> = Arc::pin(MyType);
    x.with_pin_arc_receiver();
}
