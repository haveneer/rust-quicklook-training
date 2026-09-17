use rs::identify_memory_section;
use std::sync::atomic::AtomicI32;

// Immutable `static`: its bytes can never change => placed in read-only data
// (`.rodata` on Linux, `__TEXT,__const` on macOS).
static GLOBAL_INIT: i32 = 42; // RODATA

// Also immutable from Rust's point of view (no `mut`), but `AtomicI32` wraps an
// `UnsafeCell` (interior mutability): `GLOBAL_ATOMIC.store(..)` modifies it through
// a shared `&`. The compiler knows it and must put it in writable memory
// (`.data` on Linux, `__DATA,__data` on macOS) — and no `unsafe` needed.
static GLOBAL_ATOMIC: AtomicI32 = AtomicI32::new(42); // DATA
static mut GLOBAL_UNINIT: i32 = 0; // BSS

struct Empty; // zero-sized type (ZST)

/// Prints the address of `obj_ref` and the memory section it lives in.
fn locate<T>(label: &str, obj_ref: &T) {
    let section = identify_memory_section(obj_ref);
    println!("{label:6}: {obj_ref:>12p} => {section:?}");
}

fn main() {
    let x = 10;
    let heap_num = Box::new(5);

    // `main` alone is a *function item*: a ZST, so `&main` is NOT the code address.
    // `main as fn()` coerces it to a real function pointer; the `unsafe` only builds
    // a `&u8` to pass that address along (never read through).
    let main_ptr = main as fn() as *const u8;
    locate("TEXT", unsafe { &*main_ptr });
    locate("RODATA", &GLOBAL_INIT);
    locate("DATA", &GLOBAL_ATOMIC);
    // `&GLOBAL_UNINIT` builds a shared &-reference to a `static mut`: rustc's
    // `static_mut_refs` lint flags this as unsound (another thread could mutate it
    // while the reference is alive). Kept on purpose (sound here: single-threaded,
    // never mutated). Lint-free alternative: `&raw const GLOBAL_UNINIT` (raw-borrow
    // operator, stable since 1.82) only takes the ADDRESS as a `*const i32`, with no
    // reference/aliasing guarantee, then `unsafe { &*ptr }` to get back a `&i32`.
    #[allow(static_mut_refs)]
    let bss_ref = unsafe { &GLOBAL_UNINIT };
    locate("BSS", bss_ref);
    locate("STACK", &x);
    locate("HEAP", &*heap_num);
    // A ZST occupies no memory: its reference is a dangling (but valid) address
    // equal to its alignment (0x1), outside any mapped region => None.
    let empty = Box::new(Empty); // no allocation for a ZST, even in a Box
    locate("ZST", &*empty);
}
