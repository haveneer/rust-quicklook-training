# cxx-cmake-interface

C++ ↔ Rust binding where **CMake drives the build** and the **C++ → Rust
mapping is generated**, not written by hand. Compare with
`../cxx-interface`:

| | `cxx-interface` | `cxx-cmake-interface` |
|---|---|---|
| build driven by | cargo (`build.rs` + `cxx-build`) | CMake ([Corrosion](https://github.com/corrosion-rs/corrosion)) |
| `main()` | Rust | C++ |
| who uses whom | Rust uses a C++ library | C++ uses a Rust library, on its own C++ objects |
| C++ → Rust mapping | `#[cxx::bridge]` written by hand | generated from the header by [autocxx](https://google.github.io/autocxx/) |
| typical use case | Rust crate wrapping a C++ library | introducing Rust in an existing C++ code base |

- `cpp/field.hpp` — plain C++ class `sim::Field` (name + samples), knowing
  nothing about Rust;
- `src/lib.rs` — `include_cpp! { generate!("sim::Field") }`: autocxx parses
  the header (bindgen/libclang) and generates the `cxx` bridge: constructor
  (`Field::new(n).within_unique_ptr()`), methods, `std::string`,
  `std::unique_ptr`, `const std::vector<double>&` → `&CxxVector<f64>` →
  `&[f64]` (zero copy);
- `src/bridge.rs` — autocxx only maps C++ → Rust: exporting Rust functions
  to C++ still needs a small `#[cxx::bridge]` (`extern "Rust"`), reusing the
  generated type;
- `cpp/main.cpp` — the C++ application:
  1. `rs::smooth(std::unique_ptr<Field>)` — ownership C++ → Rust → C++, the
     same object comes back, modified in place;
  2. `rs::gradient(const Field&)` — Rust borrows the field and returns a
     *new* C++ object;
  3. `rs::stats(const Field&)` — shared struct `Stats` returned by value;
  4. `Result<T, E>` on the Rust side → `rust::Error` exception in C++.

```shell
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build
./build/demo
```

How CMake drives it (`CMakeLists.txt`):

- `corrosion_import_crate(...)` runs `cargo build` (hence `build.rs`, where
  `autocxx-build` generates the C++ → Rust side and compiles its C++ glue)
  and exposes the crate as the CMake target `sim_rs` (a `staticlib`); the
  crate is a member of the `code/rs` workspace, hence `CRATES
  cxx-cmake-interface` (otherwise Corrosion imports and builds every
  workspace package);
- `corrosion_add_cxxbridge(...)` runs `cxxbridge` on `src/bridge.rs` to
  generate `sim_rs_bridge/bridge.h` / `.cpp` and `rust/cxx.h`, compiled into
  the C++ library `sim_rs_bridge` (a `cxxbridge-cmd` matching the locked
  `cxx` version is `cargo install`ed into the build directory, unless one is
  already on the `PATH`);
- the dependency is circular (C++ → Rust → C++), hence
  `sim_rs_bridge → field`.

Requirements and caveats:

- network access on first build (Corrosion, `cxxbridge-cmd`);
- **libclang** (for autocxx; set `LIBCLANG_PATH` if it isn't found);
- the header must be plain C++: a header including `rust/cxx.h` makes
  autocxx's bindgen panic;
- autocxx 0.30 bug: a constructor taking a `std::string` in a namespaced
  class generates code that doesn't compile (`cannot find trait
  ToCppString`), hence `Field(size_t)` + `set_name()`;
- passing a `std::string` from Rust needs `let_cxx_string!`;
- the generated code triggers a few `clippy` warnings (`needless_lifetimes`);
- macOS: Corrosion hard-codes
  `-L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib`; when that
  SDK doesn't match the active toolchain (e.g. Xcode), the link fails with
  `tapi error: malformed file`, so `CMakeLists.txt` drops that directory.
