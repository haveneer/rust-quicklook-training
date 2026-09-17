# Benchmarks

* `cargo bench` runs the criterion benches (wall-clock time)
* `cargo bench --features iai` runs the gungraun (ex iai-callgrind) benches (instruction counts via Valgrind);
  benches with a gungraun variant: `iai_demo` (always gungraun), `dispatch`, `block_parser`'s `parser`

## Running gungraun benches with Docker

gungraun needs Valgrind and `gungraun-runner` (same version as the `gungraun` crate), provided by
[environment/Dockerfile](environment/Dockerfile). From `code/rs`:

```shell
docker build -t rust-quicklook-bench benches/environment
docker run --rm --security-opt seccomp=unconfined \
  -v "$PWD":/work -v rq-target:/target -v rq-cargo:/usr/local/cargo/registry \
  -e CARGO_TARGET_DIR=/target -w /work rust-quicklook-bench \
  bash -c 'cargo bench --bench iai_demo &&
           cargo bench --bench dispatch --features iai &&
           cargo bench -p block_parser --bench parser --features iai'
```

* `--security-opt seccomp=unconfined` is required: gungraun disables ASLR (`setarch`), which Docker's default
  seccomp profile forbids (`setarch: failed to set personality ...: Operation not permitted`)
* `rq-target` and `rq-cargo` volumes keep build artifacts and crates between runs; a new run is compared to the
  previous one (first run shows `N/A`)
* instruction counts depend on the architecture (aarch64 on Apple Silicon vs x86_64)
