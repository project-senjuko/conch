FROM debian:bullseye-slim
COPY target/release/conch /usr/local/bin/conch
COPY dashboard/dist/spa /usr/local/bin/dashboard
WORKDIR /usr/local/bin
CMD ["conch"]
