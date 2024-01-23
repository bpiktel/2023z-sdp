FROM node:18-alpine AS frontend-build

ARG BASE_API_URL

WORKDIR /app
COPY ./frontend .
RUN npm install

COPY . .

ENV VITE_BASE_API_URL=$BASE_API_URL
RUN npm run build

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
COPY --from=frontend-build /app/dist/ /app/static/
COPY --from=backend-build /app/config/ /app/config/
ENTRYPOINT ["/app/backend"]
