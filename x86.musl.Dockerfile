FROM --platform=linux/amd64 rust:alpine AS builder

WORKDIR /app

COPY src src
COPY Cargo.toml Cargo.toml

RUN apk update && apk upgrade && apk add musl-dev
RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM --platform=linux/amd64 scratch

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/vackup /app/vackup

ENTRYPOINT [ "/app/vackup" ]