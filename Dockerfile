FROM rust:nightly AS builder

WORKDIR /app

COPY . /app/.

RUN --mount=type=cache,id=cargo-git,target=/root/.cargo/git \
    --mount=type=cache,id=cargo-registry,target=/root/.cargo/registry \
    --mount=type=cache,id=cargo-target,target=/app/target \
    cargo build --release --target x86_64-unknown-linux-musl