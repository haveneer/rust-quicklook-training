use std::env;

fn main() {
    let rust_toolchain = env::var("RUSTUP_TOOLCHAIN").unwrap();
    #[allow(clippy::if_same_then_else)]
    if rust_toolchain.starts_with("stable") {
        // do nothing
    } else if rust_toolchain.starts_with("1.85") {
        // do nothing
    } else if rust_toolchain.starts_with("nightly") {
        //enable the 'nightly' feature flag
        println!("cargo:rustc-cfg=feature=\"nightly\"");
    } else {
        panic!("Unexpected value for rustc toolchain: {rust_toolchain}")
    }
}
