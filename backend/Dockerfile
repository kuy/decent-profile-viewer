FROM rust:1.70.0-slim AS builder
WORKDIR /apps

RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libssl-dev && \
    apt-get -y clean && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
RUN mkdir /apps/src && \
    touch ./src/lib.rs && \
    cargo build --release

COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    apt-get -y clean && rm -rf /var/lib/apt/lists/*
COPY --from=builder /apps/target/release/profile-viewer-api /apps/api
CMD ["/apps/api"]
