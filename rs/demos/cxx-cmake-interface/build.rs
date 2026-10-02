// autocxx generates the C++ -> Rust bindings from cpp/field.hpp (needs libclang)
// and compiles its own C++ glue into this crate.
fn main() {
    let cpp = std::path::PathBuf::from("cpp");
    let mut clang_args = vec!["-std=c++17".to_string()];
    // A Homebrew libclang (llvm@N in PATH) finds its own libc++ headers but not the
    // system C headers (stdlib.h, wchar.h, math.h...) unless told where the SDK is.
    if cfg!(target_os = "macos") {
        let sdk = std::env::var("SDKROOT")
            .ok()
            .filter(|s| !s.is_empty())
            .or_else(|| {
                let out = std::process::Command::new("xcrun")
                    .args(["--show-sdk-path"])
                    .output()
                    .ok()?;
                out.status
                    .success()
                    .then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
            });
        if let Some(sdk) = sdk {
            clang_args.push("-isysroot".to_string());
            clang_args.push(sdk);
        }
    }
    let clang_args: Vec<&str> = clang_args.iter().map(String::as_str).collect();
    autocxx_build::Builder::new("src/lib.rs", [&cpp])
        .extra_clang_args(&clang_args)
        .build()
        .unwrap()
        .flag_if_supported("-std=c++17")
        .compile("sim-autocxx");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=cpp/field.hpp");
    println!("cargo:rerun-if-env-changed=SDKROOT");
}
