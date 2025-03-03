#![cfg_attr(feature = "nightly", feature(negative_impls))]
#![allow(dead_code)]

struct TrustMeThisTypeIsSafe {
    marker: std::marker::PhantomData<*const ()>, // Contient un pointeur brut
}

unsafe impl Send for TrustMeThisTypeIsSafe {}

struct MyTypeIsNotSafe {
    hardware_address: usize,
}

#[cfg(feature = "nightly")]
impl !Send for MyTypeIsNotSafe {} // not yet stabilized

#[test]
fn test() {
    let x = TrustMeThisTypeIsSafe {
        marker: std::marker::PhantomData,
    };
    std::thread::spawn(move || drop(x));

    let x = MyTypeIsNotSafe {
        hardware_address: 0,
    };
    #[cfg(not(feature = "nightly"))]
    std::thread::spawn(move || drop(x));
}
