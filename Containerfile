FROM rust:alpine AS builder

RUN set -x && \
    apk add --no-cache musl-dev openssl-dev openssl-libs-static

ENV OPENSSL_STATIC=1

WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new rustlet
WORKDIR /usr/src/rustlet
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .


FROM scratch

LABEL org.opencontainers.image.base.name="scratch"
LABEL org.opencontainers.image.ref.name="rustlet"
LABEL org.opencontainers.image.authors="Mehdi Jr-Gr"
LABEL org.opencontainers.image.title="Rustlet"
LABEL org.opencontainers.image.description="Rustlet is a lightweight, blazing-fast **init container** tool built in Rust"
LABEL org.opencontainers.image.version="1.0.0"
LABEL org.opencontainers.image.vendor="MJG"

USER 1000

COPY --from=builder --chmod=0755 /usr/local/cargo/bin/rustlet /rustlet

ENTRYPOINT ["/rustlet"]