FROM rust:alpine AS builder

RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static

ENV OPENSSL_STATIC=1 \
    OPENSSL_DIR="/usr"

WORKDIR /usr/src/

RUN rustup target add x86_64-unknown-linux-musl
RUN USER=root cargo new rustlet
WORKDIR /usr/src/rustlet

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch

LABEL org.opencontainers.image.base.name="scratch" \
      org.opencontainers.image.ref.name="rustlet" \
      org.opencontainers.image.authors="Mehdi Jr-Gr" \
      org.opencontainers.image.title="Rustlet" \
      org.opencontainers.image.description="Lightweight init container tool for Kubernetes checks" \
      org.opencontainers.image.vendor="Mehdi Jr-Gr" \
      org.opencontainers.image.licenses="Apache-2.0"

USER 1000:1000

COPY --from=builder --chmod=0755 \
     /usr/src/rustlet/target/x86_64-unknown-linux-musl/release/rustlet \
     /rustlet

ENTRYPOINT ["/rustlet"]