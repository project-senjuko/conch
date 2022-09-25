FROM alpine:3.16.2
COPY ./target/release/conch /conch/conch
CMD chmod +x /conch/conch && /conch/conch
