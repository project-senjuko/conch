FROM alpine:3.16.2
COPY ./target/release/conch /bin/conch
CMD chmod +x /bin/conch && conch
