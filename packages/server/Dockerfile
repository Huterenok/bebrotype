FROM rust:1.60 as build

RUN USER=root cargo new --bin server
WORKDIR /server

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/server*
RUN cargo build --release

FROM debian:buster-slim
RUN mkdir /server
WORKDIR /server
COPY --from=build /server/target/release/server .

CMD ["./server"]