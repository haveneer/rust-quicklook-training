`nightly` toolchain set using `toolchain` root project file

To run unstable tests, use:
```
cargo +nightly test --features=nightly --test <TEST_FILENAME> -- --nocapture
```