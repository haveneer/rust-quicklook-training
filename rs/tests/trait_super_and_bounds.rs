#![allow(dead_code)]

trait AdvancedFeatures {
    fn do_advanced_stuff(&self);
}

trait SuperTrait {}

// SuperTrait will be required on any type that implements Foo
trait Foo: SuperTrait {
    fn do_basic(&self);
    fn do_advanced(&self)
    where
        Self: AdvancedFeatures, // conditionally available
    {
        println!("do_advanced on Foo");
        self.do_advanced_stuff();
    }
}

struct Simple;
impl SuperTrait for Simple {} // Mandatory
impl Foo for Simple {
    fn do_basic(&self) {
        println!("do_basic on Simple");
    }

    // Not required since it has a default implementation
    // If Self (aka Simple) doesn't implement AdvancedFeatures
    //    the following method will not be available
    // fn do_advanced(&self) where Self: AdvancedFeatures { unimplemented!() }
}

struct Complete;
impl SuperTrait for Complete {} // Mandatory
impl AdvancedFeatures for Complete {
    // Optional impl to enable methods with Self: AdvancedFeatures
    fn do_advanced_stuff(&self) {
        println!("do_advanced_stuff on Complete");
    }
}

impl Foo for Complete {
    fn do_basic(&self) {
        println!("do_basic on Complete");
    }

    // Since Complete implements AdvancedFeatures, either Complete can reuse the default impl
    //   or define a new one
    // fn do_advanced(&self) where Self: AdvancedFeatures { println!("do_advanced on Complete"); }
}

fn main() {
    Simple.do_basic();
    // Simple.do_advanced(); // not available since Simple doesn't implement AdvancedFeatures

    Complete.do_basic();
    Complete.do_advanced(); // Ok
}

#[test]
fn test() {
    main()
}
