FROM rust:slim-buster as builder
WORKDIR /code

RUN user=root cargo init
COPY Cargo.toml Cargo.toml
RUN cargo fetch

COPY src src

RUN cargo build --release

FROM ubuntu:latest
WORKDIR /app

COPY --from=builder /code/target/release/menhera_album menhera_album
COPY assets assets

USER 666

EXPOSE 8080

CMD [ "/app/menhera_album" ]