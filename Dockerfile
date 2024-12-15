FROM rust:nightly AS builder

WORKDIR /app

COPY . /app/.

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --target x86_64-unknown-linux-musl