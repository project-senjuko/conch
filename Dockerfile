FROM debian:bullseye
COPY ./target/release/conch /bin/conch
CMD chmod +x /bin/conch && conch
