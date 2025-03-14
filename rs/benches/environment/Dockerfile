# Rust stable basé sur Debian bullseye
FROM rust:1.85-bookworm

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    valgrind \
    linux-perf \
    && rm -rf /var/lib/apt/lists/*

# Installer cargo-expand
RUN cargo install cargo-expand

# Installer criterion (pour cargo bench)
RUN cargo install cargo-criterion

# Installer iai-callgrind
RUN cargo install iai-callgrind-runner


# (Optionnel) Installer Miri
# RUN rustup +nightly component add miri

