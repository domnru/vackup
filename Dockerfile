FROM rust AS builder
WORKDIR /source

COPY src src
COPY Cargo.toml Cargo.toml

RUN cargo build --release

FROM debian AS runner
WORKDIR /app

RUN apt update
RUN apt install p7zip-full -y

COPY --from=builder /source/target/release/vackup /app/vackup

CMD [ "/app/vackup" ]
