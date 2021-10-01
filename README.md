# `cb-rust-demo`

This is a demonstration of Rust in the context of web applications (to be) presented at
TEX at Codeborne offices.

## API Documentation

SwaggerUI that you can play with is available at [http://cb-rust-demo.herokuapp.com/openapi](http://cb-rust-demo.herokuapp.com/openapi)

## Local environment setup

The application will run on port 8080.

```bash
$ cargo run
```

## Building a docker container

```bash
$ docker build -t cb_rust_demo .
$ docker run -it --rm -p 8080:8080 cb_rust_demo
```
