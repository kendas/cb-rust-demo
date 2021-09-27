FROM rust:alpine AS build

RUN USER=root cargo new --bin app
WORKDIR /app

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN apk add --no-cache musl-dev
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/cb_rust_demo*
RUN cargo build --release

# peoduce final image
FROM alpine

RUN mkdir /app
COPY ./openapi /app/openapi/
COPY --from=build /app/target/release/cb-rust-demo /usr/local/bin/cb-rust-demo

EXPOSE 8080

WORKDIR /app
CMD cb-rust-demo
