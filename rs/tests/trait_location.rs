mod my_trait {
    pub trait MyTrait {
        fn f();
    }
}

mod my_struct {
    pub struct MyStruct;
}

mod my_impl {
    // could be done also in a tests module for specific debugging purposes
    impl super::my_trait::MyTrait for super::my_struct::MyStruct {
        fn f() {}
    }
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        // help: items from traits can only be used if the trait is in scope
        // help: the following trait is implemented but not in scope
        use my_trait::MyTrait as _; // if it can cause symbol collision
        //                          // you can mask it with an alias

        my_struct::MyStruct::f();
    }
}