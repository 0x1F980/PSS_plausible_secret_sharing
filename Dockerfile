# Dockerfile — static musl pss binary (scratch image)
FROM rust:1.78.0-slim AS builder

WORKDIR /usr/src/pss
COPY . .

RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && apt-get install -y musl-tools && \
    cargo build --release --bin pss --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /usr/src/pss/target/x86_64-unknown-linux-musl/release/pss /pss
ENTRYPOINT ["/pss"]
