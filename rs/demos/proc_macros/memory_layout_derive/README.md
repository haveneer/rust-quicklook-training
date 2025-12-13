### memory_layout_derive — `#[derive(MemoryLayout)]`

This derive macro generates an inherent method `print_layout()` for structs with named fields. Calling it prints a readable memory layout of the type: total size, alignment, field offsets/sizes, and any padding between fields.

#### Supported
- Structs with named fields (e.g., `struct Foo { a: u8, b: u16 }`).
- Generic structs are fine; type names are printed via `core::any::type_name`.

#### Not supported
- Tuple or unit structs.
- Enums.
- If used on an unsupported item, compilation fails with a clear `compile_error!` message.

---

### Usage

Add the derive to your struct and call the generated method:

```rust
use memory_layout_derive::MemoryLayout;

#[derive(MemoryLayout, Debug, Default)]
struct Small {
    a: u8,
    b: u16,
    c: u32,
}

fn main() {
    Small::print_layout();
}
```

Example output (will vary by target/platform):

```
Memory layout for Small:
  Total size: 8 bytes
  Alignment: 4 bytes

  [padding]            @   0  (  0 bytes)
  a                    @ offset=  0  (  1 bytes)  [orig:  1]  type=u8
  [padding]            @ offset=  1  (  1 bytes)
  b                    @ offset=  2  (  2 bytes)  [orig:  2]  type=u16
  c                    @ offset=  4  (  4 bytes)  [orig:  3]  type=u32
```

Notes:
- The method prints to stdout. When running in tests, use `cargo test -- --nocapture` to see the output.
- The macro itself works on stable Rust (it uses the stabilized `core::mem::offset_of!`). Nightly is only needed for the optional compiler layout dump described below.

---

### Running inside this workspace

This crate includes tests that call `print_layout()` to exercise the macro. From the workspace root:

```
cargo test -p memory_layout_derive -- --nocapture
```

---

### Alternative: compiler-reported sizes with nightly

Rustc can print detailed size/layout information for types during compilation. This is useful for:
- Verifying struct/enum sizes as seen by the compiler
- Seeing enum variant details (which the macro above does not handle)

Requirement: use nightly and the unstable compiler flag `-Zprint-type-sizes` via `RUSTFLAGS`.

Basic usage from the workspace root (or your own crate):

```
RUSTFLAGS="-Zprint-type-sizes" cargo +nightly build -p memory_layout_derive
```

Tips:
- The output goes to stderr and is quite verbose. Pipe and filter it.
  
    Show only lines for a specific type:
    ```
    RUSTFLAGS="-Zprint-type-sizes" cargo +nightly build -p your_crate 2>&1 | grep "type: .*::YourType"
    ```
- Use `-p your_crate` (or run in that crate) to limit the build to a specific package in a workspace.
- You can also run `cargo +nightly test` with the same `RUSTFLAGS` to see sizes while building tests.

What you’ll see (examples):
- For structs: field list, size, and alignment as computed by rustc.
- For enums: per-variant details and overall size/discriminant info (this is something the derive macro here does not provide).

---

### When to use which
- Use `#[derive(MemoryLayout)]` when you want a friendly, per-field offset/size view with visible padding for a struct you control.
- Use `RUSTFLAGS="-Zprint-type-sizes" cargo +nightly build` when you need authoritative compiler layout info, especially for enums or types outside your crate, and you are okay using nightly.
