FROM alpine

RUN mkdir /app
COPY ./openapi /app/openapi/
COPY ./target/release/cb-rust-demo /usr/local/bin/cb-rust-demo

EXPOSE 8080

WORKDIR /app
CMD cb-rust-demo
