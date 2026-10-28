# Rust stable basé sur Debian bullseye
FROM rust:1.90-bookworm

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    valgrind \
    linux-perf \
    gdb \
    && rm -rf /var/lib/apt/lists/*

# Installer cargo-expand
RUN cargo install cargo-expand

# Installer criterion (pour cargo bench)
RUN cargo install cargo-criterion

# Installer gungraun (ex iai-callgrind)
RUN cargo install gungraun-runner --version 0.20.0 --locked # must match the gungraun crate version


# (Optionnel) Installer Miri
# RUN rustup +nightly component add miri

