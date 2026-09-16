// public module: mylib1::f3::f3()
pub mod f3;
// private module: mylib1::f4::f4() is refused
mod f4;

// ...but its content can be re-exported: mylib1::f4()
pub use f4::f4;
// ...possibly renamed: mylib1::f5()
pub use f4::f4 as f5;

// private module
mod f6x;

// re-export all its public symbols: mylib1::f6a()
pub use f6x::*;
