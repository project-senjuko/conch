FROM rust:1-bullseye as builder
WORKDIR /usr/src/conch
COPY . .
RUN cargo build --bin conch --release

FROM debian:bullseye-slim
COPY --from=builder /usr/src/conch/target/release/conch /usr/local/bin/conch
COPY dashboard/dist/spa /usr/local/bin/dashboard
WORKDIR /usr/local/bin
CMD ["conch"]
