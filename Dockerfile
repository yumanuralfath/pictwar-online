FROM rust:nightly AS builder

WORKDIR /app

COPY . /app/.

RUN --mount=type=cache,id=cache-cargo-git,target=/root/.cargo/git \
    --mount=type=cache,id=cache-cargo-registry,target=/root/.cargo/registry \
    --mount=type=cache,id=cache-cargo-target,target=/app/target \
    cargo build --release --target x86_64-unknown-linux-musl