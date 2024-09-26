FROM rust:1.81 AS backend-build
RUN apt-get update && apt-get upgrade -y && apt-get install libclang-dev -y
WORKDIR /app/
COPY ./ /app/
RUN --mount=type=cache,target=/usr/local/cargo/registry/index \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    mv /app/target/release/backend /app/

FROM debian:bookworm-slim AS runtime
WORKDIR /app/
COPY --from=backend-build /app/backend /app/
COPY --from=backend-build /app/config/ /app/config/
COPY --from=backend-build /app/migrations/ /app/migrations/
ENTRYPOINT ["/app/backend"]
