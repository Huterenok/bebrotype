FROM rust:1-slim-bullseye AS builder
WORKDIR /code

RUN USER=root apt update
RUN USER=root apt install libssl-dev pkg-config libpq-dev -y

COPY Cargo.toml .
COPY .env .
RUN cargo fetch

COPY src ./src
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app

RUN USER=root apt update
RUN USER=root apt install openssl libpq5 -y

COPY --from=builder /code/target/release/server .
COPY .env .

USER 1001

EXPOSE 3001

CMD [ "/app/server" ]