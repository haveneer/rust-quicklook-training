# inventory vs linkme: Distributed Plugin Registration in Rust

This project demonstrates two popular Rust crates for distributed plugin registration: `inventory` and `linkme`. Both enable decentralized registration of items across modules without maintaining a central list, but they work differently under the hood.

## Quick Start

```bash
# Run the inventory example
cargo run --bin inventory_example

# Run the linkme example
cargo run --bin linkme_example
```

## Overview

Both crates solve the same problem: allowing different modules to register plugins, commands, or extensions without requiring a central registration file. This is particularly useful in large projects where maintaining a central list would create merge conflicts and tight coupling.

## Key Differences

| Feature | `inventory` | `linkme` |
|---------|-------------|----------|
| **Mechanism** | Runtime initialization | Linker-based (compile-time) |
| **Data Structure** | Iterator | Static slice `[T]` |
| **Performance** | Small runtime cost | Zero runtime cost |
| **Type Requirements** | Any type | Must be `Sync` |
| **Flexibility** | More flexible | More restrictive |
| **Platform Support** | Broad (incl. WASM) | Requires specific linker features |
| **Access Pattern** | `inventory::iter::<T>()` | Direct slice access |

## How They Work

### inventory

- Uses module constructors that run before `main()`
- Builds the registry at **runtime startup**
- Returns an iterator over collected items
- More portable across platforms

**Syntax:**
```rust
inventory::collect!(PluginType);

inventory::submit! {
    PluginType { /* ... */ }
}

for item in inventory::iter::<PluginType> {
    // use item
}
```

### linkme

- Uses linker sections to collect items at **link time**
- Creates a static slice directly in the binary
- Zero overhead: items are contiguous in memory
- Requires linker support (works on major platforms)

**Syntax:**
```rust
#[distributed_slice]
pub static PLUGINS: [PluginType];

#[distributed_slice(PLUGINS)]
static ITEM: PluginType = /* ... */;

for item in PLUGINS {
    // use item
}
```

## Similarities

Both crates provide:
- ✅ Decentralized registration from any module
- ✅ No central list to maintain
- ✅ Type-safe collections
- ✅ Works across compilation units
- ✅ Reduces merge conflicts in team projects
- ✅ Enables plugin architectures

## When to Use Each

### Use `inventory` when:
- You need maximum portability (including WebAssembly)
- Your items are complex types with non-trivial initialization
- You want to support dynamic linking scenarios
- Runtime flexibility is more important than performance

### Use `linkme` when:
- Performance is critical (zero runtime overhead)
- You're targeting standard platforms (Linux, macOS, Windows, BSD)
- Your items can be `const` or simple static data
- You want compile-time guarantees about the collection

## Example Use Cases

Both crates are excellent for:
- **CLI frameworks**: Register subcommands from different modules
- **Test frameworks**: Collect test cases automatically
- **Plugin systems**: Register plugins without central coordination
- **Benchmark suites**: Gather benchmarks from multiple modules
- **Configuration**: Collect validators, parsers, or handlers

## Code Comparison

The examples in this project demonstrate the same command system implemented with both crates. Notice:

1. **Structure**: Both use similar patterns with `#[distributed_slice]` vs `inventory::collect!`
2. **Registration**: `#[distributed_slice(SLICE)]` vs `inventory::submit!`
3. **Access**: Direct slice iteration vs `inventory::iter::<T>()`
4. **Syntax**: `linkme` requires static items, `inventory` can use expressions

## Recommendations

- **Start with `inventory`**: Easier to use, more forgiving
- **Optimize with `linkme`**: If profiling shows the need, and platform constraints allow
- **Hybrid approach**: Use both where appropriate (they can coexist)

## Further Reading

- [`inventory` documentation](https://docs.rs/inventory)
- [`linkme` documentation](https://docs.rs/linkme)
- [dtolnay's blog on distributed registration](https://github.com/dtolnay)
