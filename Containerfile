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


LABEL org.opencontainers.image.base.name="scratch"
LABEL org.opencontainers.image.description="Lightweight init container tool for Kubernetes checks"
LABEL org.opencontainers.image.ref.name="rustlet"
LABEL org.opencontainers.image.authors="Mehdi Jr-Gr"
LABEL org.opencontainers.image.title="Rustlet"
LABEL org.opencontainers.image.vendor="Mehdi Jr-Gr"
LABEL org.opencontainers.image.source="https://github.com/mjrgr/rustlet"
LABEL org.opencontainers.image.licenses="Apache-2.0"

USER 1000:1000

COPY --from=builder --chmod=0755 \
     /usr/src/rustlet/target/x86_64-unknown-linux-musl/release/rustlet \
     /rustlet

ENTRYPOINT ["/rustlet"]