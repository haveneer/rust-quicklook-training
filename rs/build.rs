use std::env;

fn main() {
    let rust_toolchain = env::var("RUSTUP_TOOLCHAIN").unwrap();
    if rust_toolchain.starts_with("stable") {
        // do nothing
    } else if rust_toolchain.starts_with("nightly") {
        //enable the 'nightly' feature flag
        println!("cargo:rustc-cfg=feature=\"nightly\"");
    } else {
        panic!("Unexpected value for rustc toolchain")
    }
}
