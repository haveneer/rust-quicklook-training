[![C++ CI](https://github.com/haveneer/rust-quicklook-training/workflows/C++%20CI/badge.svg)](https://github.com/haveneer/rust-quicklook-training/actions) 
[![Rust CI](https://github.com/haveneer/rust-quicklook-training/workflows/Rust%20CI/badge.svg)](https://github.com/haveneer/rust-quicklook-training/actions) 

<p xmlns:dct="http://purl.org/dc/terms/" xmlns:cc="http://creativecommons.org/ns#" class="license-text">
    <a rel="license" href="https://creativecommons.org/licenses/by-nc-sa/4.0">
        <img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-nc-sa/4.0/88x31.png" />
    </a><br>
    <a rel="cc:attributionURL" property="dct:title" href="https://github.com/haveneer/rust-quicklook-training">
        Code snippets for Rust Quicklook Training
    </a>
    by 
    <a rel="cc:attributionURL dct:creator" property="cc:attributionName" href="mailto:hpwxf@haveneer.com">
        Pascal HAVÉ
    </a> is licensed under 
    <a rel="license" href="https://creativecommons.org/licenses/by-nc-sa/4.0">CC BY-NC-SA 4.0</a>
</p>

For more details about this training, contact [hpwxf@haveneer.com](mailto:hpwxf@haveneer.com).

# Rust Quicklook Training 

## Rust part

Use conventional Rust build using `cargo`

```
cargo build [--release]
cargo test
cargo bench
cargo run --example julia
```

## C++ part

* Any file `filename.cpp` is automatically compiled using extra files `filename.h*` `filename--*.*`

* Any file `filaname.cxx` is not compiled and usually contains explicit bugs 

* `type.h` contains tool `type(obj)` to get printable type name for given object argument 

# CMake options

* `-DENABLE_STATIC_ANALYSIS=ON|OFF` : enable/disable static analysis while compiling
 
* `-DCMAKE_CXX_COMPILER_LAUNCHER=ccache` : enable `ccache` as compiler cache


