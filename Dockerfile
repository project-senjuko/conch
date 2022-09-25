FROM ubuntu:22.10
COPY ./target/release/conch /bin/conch
CMD chmod +x /bin/conch && conch
