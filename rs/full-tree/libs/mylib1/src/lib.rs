// declare f3.rs as a module and allow to navigate into in from outside
pub mod f3;
// declare f4.rs as a _private_ module
mod f4;

// allows using f4::f4 as this_crate::f4 from outside
pub use f4::f4;
// allows using f4::f4 as this_crate::f5 from outside
pub use f4::f4 as f5;

// declare f6x.rs as a _private_ module and
mod f6x;

// export all public symbols from f6x as this_crate:: symbols
pub use f6x::*;
