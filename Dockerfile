FROM rust:slim-bullseye as builder
WORKDIR /code

RUN user=root cargo init
COPY Cargo.toml Cargo.toml
RUN cargo fetch

COPY src src

RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app

COPY --from=builder /code/target/release/menhera_album menhera_album
COPY assets assets

USER 666

EXPOSE 8080

CMD [ "/app/menhera_album" ]