FROM rust:slim-buster as builder
WORKDIR /code

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

RUN user=root cargo init
COPY Cargo.toml Cargo.toml
RUN cargo fetch

COPY src src

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch
WORKDIR /app

COPY --from=builder /code/target/x86_64-unknown-linux-musl/release/menhera_album menhera_album
COPY assets assets

USER 666

EXPOSE 8080

CMD ["/app/menhera_album"]