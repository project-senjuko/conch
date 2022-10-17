FROM rust:1-slim as builder
WORKDIR /usr/src/conch
COPY . .
ENV SJKCONCH_MAINTAINER_NAME=github-actions SJKCONCH_MAINTAINER_EMAIL=<unsupported>
RUN apt update && apt install -y musl-tools && rustup target add x86_64-unknown-linux-musl && cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3
COPY --from=builder /usr/src/conch/target/x86_64-unknown-linux-musl/release/conch /usr/local/bin/conch
CMD ["conch"]
