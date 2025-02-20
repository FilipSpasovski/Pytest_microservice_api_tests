# Backend_service

This repository provide a rust http micro service that embed a "pseudo-ml" model.

## Build

```rust
cargo build
```

## Run

```rust
cargo run
```

## Tests

```rust
cargo test
```

## Docker

To build a docker image wrapping the application, you can launch.

```sh
docker build -t my_service_name -f Dockerfile .
```

To run it

```sh
docker run --rm -e APP_HOST="0.0.0.0" -e APP_PORT=3000 -p 3000:3000 my_service_name
```

You can test that it's working with:

```sh
curl -v -X GET http://0.0.0.0:3000/status
```

You should receive a 200 OK response.
