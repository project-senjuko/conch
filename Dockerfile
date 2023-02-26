FROM debian:bullseye-slim
COPY target/release/senjuko-conch /usr/local/bin/conch
COPY dashboard/dist/spa /usr/local/bin/dashboard
WORKDIR /usr/local/bin
CMD ["conch"]
