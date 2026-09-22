# c-interface

Raw C ↔ Rust FFI, without a bridging crate like `cxx` (compare with
`../cxx-interface`, which is the C++ equivalent using `cxx`). `c/clib.c` /
`c/clib.h` are a small plain-C library, compiled by `build.rs` via the `cc`
crate and linked into this binary.

Four cases, each in `src/ffi/`:

1. **Struct by pointer** — `Point` (`#[repr(C)]`) passed as `const Point*`.
2. **Pointer + length** — `sum_array(const double*, size_t)`, the classic C
   array idiom (slice ↔ raw pointer).
3. **Opaque handle + `Drop`** — `Counter*` created/freed on the C side,
   wrapped in a Rust type whose `Drop` impl calls `counter_free`
   automatically (ownership crossing the FFI boundary).
4. **Callback (bonus)** — C calls back into an arbitrary Rust `FnMut(i32)`
   through a generic `extern "C" fn` trampoline.

```shell
cargo run -p c-interface
```

`src/ffi/manual.rs` hand-writes the `extern "C"` declarations. That's common
for small, stable APIs and teaching, but isn't the industrial mainstream for
real C libraries — the standard production pattern is `cc` (build the C
source) + `bindgen` (generate the bindings from the header) + a safe Rust
wrapper. To see that path too:

```shell
cargo run -p c-interface --features bindgen-compare
```

This generates the same bindings via `bindgen` (`src/ffi/generated.rs`) and
prints a comparison against the hand-written ones. Requires `libclang`
installed locally, which is why it's opt-in rather than the default.

Further reading: <https://rust-embedded.github.io/book/interoperability/c-with-rust.html>
