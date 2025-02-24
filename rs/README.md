`nightly` toolchain set using `toolchain` root project file

To run unstable tests, use:
```
cargo +nightly test --test <TEST_FILENAME> -- --nocapture
```

PS: `nightly` feature is enabled automatically by default when using a nightly version (cf [`build.rs`](build.rs)).