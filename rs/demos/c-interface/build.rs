fn main() {
    cc::Build::new()
        .file("c/clib.c")
        .include("c")
        .compile("clib");

    println!("cargo:rerun-if-changed=c/clib.c");
    println!("cargo:rerun-if-changed=c/clib.h");

    // point_norm() uses sqrt() from <math.h>. On macOS that ships inside
    // libSystem for free; on Linux/BSD it lives in a separate libm that
    // must be linked explicitly — a real, common FFI gotcha.
    #[cfg(all(unix, not(target_os = "macos")))]
    println!("cargo:rustc-link-lib=m");

    #[cfg(feature = "bindgen-compare")]
    generate_bindgen();
}

#[cfg(feature = "bindgen-compare")]
fn generate_bindgen() {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindgen::Builder::default()
        .header("c/clib.h")
        .generate()
        .expect("bindgen failed to generate bindings from c/clib.h")
        .write_to_file(out_dir.join("bindgen.rs"))
        .expect("failed to write bindgen.rs");
}
