// autocxx generates the C++ -> Rust bindings from cpp/field.hpp (needs libclang)
// and compiles its own C++ glue into this crate.
fn main() {
    let cpp = std::path::PathBuf::from("cpp");
    autocxx_build::Builder::new("src/lib.rs", [&cpp])
        .extra_clang_args(&["-std=c++17"])
        .build()
        .unwrap()
        .flag_if_supported("-std=c++17")
        .compile("sim-autocxx");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=cpp/field.hpp");
}
