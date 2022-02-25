FROM rust:1.59.0-slim AS builder
WORKDIR /apps

COPY Cargo.toml Cargo.lock ./
RUN mkdir /apps/src && \
    touch ./src/lib.rs && \
    cargo build --release

COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /apps/target/release/profile-viewer-api /apps/api
CMD ["/apps/api"]
