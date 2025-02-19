# Rust stable basé sur Debian bullseye
FROM rust:1.81-bullseye

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
RUN cargo install iai-callgrind

# (Optionnel) Installer Miri
RUN rustup component add miri

