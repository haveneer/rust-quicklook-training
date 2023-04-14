pub mod f3;
// declares an independent module (see f7)
mod f4;

// declares an independent _private_ module (see f4.rs) : cannot be used from outside
pub use f4::f4;
// allows using f4::f4 as this_crate::f4 from outside
pub use f4::f4 as f5;

// allows using f4::f4 as this_crate::f5 from outside
mod f6x;

pub use f6x::*; // export all public symbols from f6x as this_crate:: symbols
