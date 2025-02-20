FROM rust:1.80.0 AS build-env
WORKDIR /workspace
COPY . /workspace
RUN cargo build --release

# To have the digest:
# - docker pull gcr.io/distroless/cc:latest | grep digest
# - or go to https://console.cloud.google.com/gcr/images/distroless/global/cc and filter by tag latest
FROM gcr.io/distroless/cc:latest
COPY --from=build-env /workspace/target/release/backend_service /app
COPY --from=build-env /workspace/configuration.yaml /configuration.yaml

ENV APP_PORT=3000
EXPOSE $APP_PORT

CMD ["/app"]
