FROM rust:1-slim as builder
WORKDIR /usr/src/conch
COPY . .
RUN apt update && apt install -y musl-tools && rustup target add x86_64-unknown-linux-musl && cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.16.2
COPY --from=builder /usr/src/conch/target/x86_64-unknown-linux-musl/release/conch /usr/local/bin/conch
CMD ["conch"]
