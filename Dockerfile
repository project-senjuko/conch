FROM rust:1-bullseye as builder
ARG MAINTAINER_NAME
ARG MAINTAINER_EMAIL
WORKDIR /usr/src/conch
COPY . .
ENV SJKCONCH_MAINTAINER_NAME=$MAINTAINER_NAME SJKCONCH_MAINTAINER_EMAIL=$MAINTAINER_EMAIL
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /usr/src/conch/target/release/conch /usr/local/bin/conch
COPY dashboard/dist/spa /usr/local/bin/dashboard
WORKDIR /usr/local/bin
CMD ["conch"]
