FROM node:lts-bullseye-slim AS frontend-build
# Tutaj trzeba zbudować frontend jako pliki statyczne

FROM rust:1.71 AS backend-build
RUN apt-get update && apt-get upgrade -y && apt-get install libclang-dev -y
WORKDIR /app/
COPY ./backend/ /app/
RUN --mount=type=cache,target=/usr/local/cargo/registry/index \
    --mount=type=cache,target=/usr/local/cargo/registry/cache \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    mv /app/target/release/backend /app/

FROM debian:bullseye-slim AS runtime
WORKDIR /app/
COPY --from=backend-build /app/backend /app/
# Tutaj trzeba podać ścieżkę do tych plików, chodzi o folder z `index.html`
# COPY --from=frontend-build <???> /app/static/
COPY --from=backend-build /app/config/ /app/config/
ENTRYPOINT ["/app/backend"]
