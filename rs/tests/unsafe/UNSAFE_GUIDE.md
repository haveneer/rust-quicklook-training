# Writing `unsafe` Rust — a progressive guide

> **Scope.** This guide is about *writing* `unsafe` Rust: the obligations and
> hazards that appear once the compiler stops enforcing memory, aliasing and
> lifetime guarantees for you. It is **not** about merely *calling* an existing
> `unsafe` function from the standard library (e.g. `std::mem::transmute`).
>
> Every section links to a small, runnable demo in this directory. Run them all
> on the pinned **stable** toolchain with:
>
> ```bash
> cargo test --test unsafe -- --nocapture
> ```
>
> Nightly is **not** required for the demos or the compile-fail cases — only for
> the optional Miri inspection of the UB demo (see §10).

## Table of contents

1. [What `unsafe` actually means](#1-what-unsafe-actually-means)
2. [Raw pointers](#2-raw-pointers)
3. [`unsafe fn` and the `# Safety` contract](#3-unsafe-fn-and-the--safety-contract)
4. [`unsafe trait` / `unsafe impl Send` & `Sync`](#4-unsafe-trait--unsafe-impl-send--sync)
5. [`static mut` and `&raw` / `addr_of!`](#5-static-mut-and-raw--addr_of)
6. [`union` field access](#6-union-field-access)
7. [`transmute`, layout and validity](#7-transmute-layout-and-validity)
8. [Building sound safe abstractions over unsafe internals](#8-building-sound-safe-abstractions-over-unsafe-internals)
9. [The safe-code soundness boundary: `cve-rs` / `rust#25860`](#9-the-safe-code-soundness-boundary-cve-rs--rust25860)
10. [Tooling: detecting UB with Miri (and sanitizers)](#10-tooling-detecting-ub-with-miri-and-sanitizers)
11. [Already in this repo](#11-already-in-this-repo)
12. [References](#references)

## 1. What `unsafe` actually means

In safe Rust, the compiler *proves* that your program has no data races, no
dangling references, no out-of-bounds accesses and no invalid values. `unsafe`
does **not** turn those checks off and let you do whatever you want. It does
something much narrower: it lets you use five extra "superpowers" that the
compiler cannot verify, and in exchange **you** take on the obligation to uphold
the invariants it can no longer check ([The Book, ch. 20][book-unsafe]):

1. dereference a raw pointer,
2. call an `unsafe fn` (or `unsafe` intrinsic),
3. implement an `unsafe trait`,
4. access or modify a mutable `static`,
5. access the fields of a `union`.

The crucial mental model: an `unsafe` block is **not** "the compiler trusts me
here so anything goes". It is a *proof obligation*. Inside it you must guarantee
the absence of **Undefined Behavior (UB)** — the normative list lives in the
Reference, ["Behavior considered undefined"][ref-ub]. UB is not "it crashes":
the optimizer is allowed to assume UB never happens, so a single UB anywhere can
silently corrupt *unrelated* parts of the program, and "it seemed to work" is
never evidence of soundness.

Two invariants worth naming up front (from the [Rustonomicon][nomicon] and [UCG][ucg]):

- **Validity invariant**: the bit pattern must be a valid value of its type at
  all times (e.g. a `bool` is `0` or `1`, a reference is non-null and aligned).
  Breaking it is *immediate* UB.
- **Safety invariant**: the higher-level contract a safe abstraction relies on
  (e.g. a `Vec`'s `len <= capacity`). You may *temporarily* break it inside an
  `unsafe` region, as long as it holds again before safe code can observe it.

## 2. Raw pointers

Demo: [`ptr_basics.rs`](./ptr_basics.rs)

*Creating* a `*const T` / `*mut T` is safe — a raw pointer is just an address
plus *provenance*. Only **dereferencing** it requires `unsafe`, because that is
where the invariants the compiler can no longer check come due. Before a
deref you must guarantee the pointer is:

- non-null,
- aligned for `T`,
- *dereferenceable*: inside a single live allocation for `size_of::<T>()` bytes,
- backed by valid **provenance** (casting an integer to a pointer does not
  invent the right to access memory — see the `std::ptr` ["Provenance"][ptr-doc]
  section), and
- pointing at a valid `T` when read.

**UB if violated:** reading out of bounds, through a dangling/freed pointer, or
a misaligned/null pointer is all UB. Pointer arithmetic that leaves the original
allocation (other than one-past-the-end, which may be *formed* but not *read*)
is UB too.

## 3. `unsafe fn` and the `# Safety` contract

Demo: [`unsafe_fn_contract.rs`](./unsafe_fn_contract.rs)

Marking a function `unsafe fn` does **not** make its body correct. It (a) forces
callers into an `unsafe { … }` block, and (b) obliges *you*, the author, to spell
out the preconditions in a `/// # Safety` doc section. The compiler never checks
that contract — it is a human-to-human agreement, and `clippy::missing_safety_doc`
will nag you if you forget to write it.

The canonical shape:

```rust
/// # Safety
/// Caller must guarantee `ptr` is non-null, aligned, points to an initialized
/// `T`, and stays valid (and unaliased) for the duration of the call.
unsafe fn read_t<T: Copy>(ptr: *const T) -> T { *ptr }
```

**UB if violated:** calling the function without satisfying the documented
preconditions. Note the `T: Copy` bound in the demo — it prevents accidentally
duplicating ownership of a non-`Copy` value (which would later double-free).

## 4. `unsafe trait` / `unsafe impl Send` & `Sync`

Demo: [`unsafe_send_sync.rs`](./unsafe_send_sync.rs)

Some traits carry guarantees the compiler cannot verify; declaring them
`unsafe trait` means *implementers* promise to uphold those guarantees, so safe
code that relies on the trait stays sound. `Send` and `Sync` are the famous
examples. They are **auto traits**, derived structurally and conservatively: a
type containing a raw pointer is automatically `!Send + !Sync`.

When you *know* a type is genuinely thread-safe (e.g. it uniquely owns its data
like `Box<T>`), you re-assert it with `unsafe impl`:

- `unsafe impl<T: Send> Send for MyBox<T> {}` — sound to **move** to another thread.
- `unsafe impl<T: Sync> Sync for MyBox<T> {}` — sound to **share** `&T` across threads.

**UB if violated:** a wrong `Send`/`Sync` claim enables data races, which are UB.
The matching compile-fail case
[`failures/*/send_not_implemented.rs`](./failures/stable/send_not_implemented.rs)
shows the *safe* default: moving an `Rc` into `thread::spawn` is rejected because
`Rc` is `!Send`.

## 5. `static mut` and `&raw` / `addr_of!`

Demo: [`static_mut_and_addr_of.rs`](./static_mut_and_addr_of.rs)

A `static mut` is a single global mutable location; every access is `unsafe`
because nothing prevents overlapping `&mut` to it (instant UB). The historical
footgun was forming a *reference* (`&mut COUNTER` / `&COUNTER`), which asserts
the full borrow rules for that reference's lifetime — almost never something you
can guarantee. Modern Rust lints this with `static_mut_refs` and pushes you to
**raw** accessors that never materialize an intermediate reference:

```rust
let p: *mut u64 = &raw mut COUNTER;       // stable since 1.82
// or, equivalently: let p = std::ptr::addr_of_mut!(COUNTER);
unsafe { *p += 1; }
```

**UB if violated:** concurrent access to the `static mut` is a data race. In real
code, prefer `AtomicUsize`, `OnceLock`, `Mutex`, etc. (see also
[`tests/static_and_const.rs`](../static_and_const.rs)).

## 6. `union` field access

Demo: [`union_demo.rs`](./union_demo.rs)

Unlike an `enum`, a `union` has **no tag** recording which field is active — all
fields share storage. *Writing* a field is safe; *reading* one is `unsafe`,
because the compiler cannot know whether the current bytes are a valid value of
that field's type. Your obligation: only read the field you last wrote, or a
field whose type can validly reinterpret those exact bytes.

**UB if violated:** reading an invalid bit pattern for the field's type — e.g. a
`bool` that is neither `0` nor `1`. (Reading a `u32`, by contrast, is always
fine: every bit pattern is a valid `u32`.)

## 7. `transmute`, layout and validity

Demo: [`transmute_pitfalls.rs`](./transmute_pitfalls.rs)

`std::mem::transmute::<A, B>` reinterprets the bytes of an `A` as a `B`. It is
the most dangerous function in the language. The compiler checks **only** that
`size_of::<A>() == size_of::<B>()` — a size mismatch is a compile error
(E0512), demonstrated by the trybuild case
[`failures/*/transmute_size_mismatch.rs`](./failures/stable/transmute_size_mismatch.rs).
Everything else is on you:

- **validity** — the bytes must be a valid `B` (transmuting `3u8` to `bool` is UB);
- **layout** — `repr(Rust)` field order/padding is unspecified, so reach for
  `#[repr(C)]` / `#[repr(transparent)]` (see [`tests/repr_c_specific.rs`](../repr_c_specific.rs),
  [`tests/repr_struct.rs`](../repr_struct.rs));
- **lifetimes / provenance** — `transmute` can silently extend a lifetime or
  fabricate a `&mut`, which is UB (this is also the *shape* of §9's soundness hole).

**Prefer safer alternatives first:** `f32::to_bits`/`from_bits`,
`u32::to_ne_bytes`/`from_ne_bytes`, `as` casts, `slice::from_raw_parts`, or plain
pointer casts. Most "real" `transmute`s are avoidable.

## 8. Building sound safe abstractions over unsafe internals

Demo: [`safe_abstraction.rs`](./safe_abstraction.rs)

This is the *point* of `unsafe`: confine it behind a small, audited boundary so
the **public** API is impossible to misuse from safe code. The textbook example
is a hand-written `split_at_mut`: returning two `&mut` slices into one buffer is
inexpressible to the borrow checker, yet sound *as long as the halves do not
overlap*. The wrapper guarantees that invariant (an in-bounds `assert!` plus
exclusive `&mut [T]` input), so the `unsafe` block can never produce aliasing
`&mut`, and the function is exported **without** the `unsafe` keyword.

The discipline to copy:

- keep the `unsafe` region tiny and accompany it with a `// SAFETY:` comment
  justifying every invariant;
- make the *type signature* enforce as much as possible (lifetimes, `&mut`);
- validate the rest at runtime (`assert!`) rather than assuming it.

Further reading: ["Learn Rust With Entirely Too Many Linked Lists"][toomanylists]
builds increasingly real data structures this way.

## 9. The safe-code soundness boundary: `cve-rs` / `rust#25860`

Demo: [`cve_rs_lifetime_expansion.rs`](./cve_rs_lifetime_expansion.rs)

The most unsettling lesson: **`unsafe` is not the only source of UB.** The demo
contains **no `unsafe` keyword at all**, yet safe code forges a dangling
reference and reads freed memory. It exploits a long-standing compiler soundness
hole in higher-ranked lifetime well-formedness, [rust-lang/rust#25860][issue25860]
(open since 2015), the same bug the [`cve-rs`][cve-rs] project uses to build
buffer overflows, use-after-free and segfaults without `unsafe`.

The shape: a function `lifetime_translator(_: &'a &'b (), v: &'b T) -> &'a T`
is sound on its own (the `&'a &'b ()` argument forces `'b: 'a`), but coercing it
to a `for<'x> fn(_, &'x T) -> &'b T` pointer and feeding it a `'static` seed lets
the compiler "expand" an arbitrary lifetime to `'static`, defeating the borrow
checker.

Because *using* the result is genuine UB, the UB-triggering test is gated
(`#[cfg(feature = "nightly")]` + `#[ignore]`) so the default `cargo test` stays
green and deterministic. Inspect it under Miri (next section).

## 10. Tooling: detecting UB with Miri (and sanitizers)

Why a tool at all? Executing UB on real hardware is unreliable — it may appear
to work, produce wrong results, or crash nondeterministically. **Miri** is an
interpreter that runs your program's MIR and turns latent UB (out-of-bounds,
use-after-free, invalid values, data races, alignment, some aliasing violations
via Stacked/Tree Borrows) into a clear, reproducible diagnostic.

Miri ships **only** with nightly (there is no stable Miri — that is the *one*
reason nightly is mentioned in this section):

```bash
rustup +nightly component add miri

# Run the whole unsafe suite under Miri:
cargo +nightly miri test --test unsafe

# Surface the §9 use-after-free as a deterministic Miri error:
cargo +nightly miri test --test unsafe -- --ignored cve_rs
```

The last command reports, deterministically:

```text
error: Undefined Behavior: constructing invalid value: encountered a dangling
reference (use-after-free)
```

Complementary tools: the runtime **sanitizers** (`-Zsanitizer=address` /
`thread` / `memory`, also nightly) catch UB at run time on real hardware, and
`cargo +nightly miri test` honors `MIRIFLAGS` (e.g. `-Zmiri-strict-provenance`).
For *most* of this guide — sound raw pointers, `unsafe fn`, `Send`/`Sync`,
`union`, safe abstractions — plain stable `cargo test` is enough; Miri is an
extra correctness check, not a requirement.

## 11. Already in this repo

`unsafe` appears, used soundly, in several existing demos worth studying
alongside this section (they are referenced rather than duplicated here):

- [`tests/global_allocator.rs`](../global_allocator.rs) — `unsafe impl GlobalAlloc`.
- [`tests/thread_safety_by_hand.rs`](../thread_safety_by_hand.rs) — `unsafe impl Send` to cross a thread boundary.
- [`tests/static_and_const.rs`](../static_and_const.rs) — `static mut` and constants.
- [`tests/repr_struct.rs`](../repr_struct.rs), [`tests/repr_c_specific.rs`](../repr_c_specific.rs),
  [`tests/repr_enum.rs`](../repr_enum.rs) — layout control behind `from_raw_parts` & friends.
- [`tests/self-referential-manually.rs`](../self-referential-manually.rs) — `Pin::get_unchecked_mut`.
- [`tests/async_waker_by_hand.rs`](../async_waker_by_hand.rs) — building a `RawWaker` by hand.

FFI is one of the unsafe "superpowers" too; the repo covers it separately under
`demos/c-interface-WIP` and `demos/cxx-interface`, so it is out of scope here.

## References

- **The Rustonomicon — The Dark Arts of Unsafe Rust**: <https://doc.rust-lang.org/nomicon/>
- **The Rust Reference — Behavior considered undefined**: <https://doc.rust-lang.org/reference/behavior-considered-undefined.html>
- **The Rust Programming Language — "Unsafe Rust"**: <https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html>
- **Unsafe Code Guidelines Reference (UCG)**: <https://rust-lang.github.io/unsafe-code-guidelines/>
- **Miri**: <https://github.com/rust-lang/miri>
- **Ralf Jung's blog — Stacked Borrows / Tree Borrows**: <https://www.ralfj.de/blog/>
- **`std::mem::transmute`**: <https://doc.rust-lang.org/std/mem/fn.transmute.html>
- **`std::ptr` (provenance, safer alternatives)**: <https://doc.rust-lang.org/std/ptr/index.html>
- **"Learn Rust With Entirely Too Many Linked Lists"**: <https://rust-unofficial.github.io/too-many-lists/>
- **rust-lang/rust#25860** — lifetime/well-formedness soundness bug: <https://github.com/rust-lang/rust/issues/25860>
- **`cve-rs`** — memory vulnerabilities in *safe* Rust: <https://github.com/Speykious/cve-rs>

[ref-ub]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[book-unsafe]: https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html
[nomicon]: https://doc.rust-lang.org/nomicon/
[ucg]: https://rust-lang.github.io/unsafe-code-guidelines/
[ptr-doc]: https://doc.rust-lang.org/std/ptr/index.html
[toomanylists]: https://rust-unofficial.github.io/too-many-lists/
[issue25860]: https://github.com/rust-lang/rust/issues/25860
[cve-rs]: https://github.com/Speykious/cve-rs
