fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/blobstore.cpp")
        .flag_if_supported("-std=c++14")
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/blobstore.cpp");
    println!("cargo:rerun-if-changed=include/blobstore.hpp");
}
