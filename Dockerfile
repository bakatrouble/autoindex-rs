FROM node:22-alpine AS frontend-builder
WORKDIR /app
RUN corepack enable
COPY frontend/package.json frontend/pnpm-lock.yaml frontend/pnpm-workspace.yaml ./
RUN --mount=type=cache,target=./node_modules  \
    pnpm i
COPY frontend ./
RUN --mount=type=cache,target=./node_modules \
    pnpm build

FROM rust:1.96-alpine AS rust-builder
WORKDIR /app
COPY . .
COPY --from=frontend-builder /app/dist/ ./frontend/dist/
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=./target \
    cargo build --release && mv ./target/release/autoindex-rs .

FROM alpine
RUN mkdir -p /app/files/root && mkdir -p /app/files/subdomains
COPY --from=rust-builder /app/autoindex-rs /app/autoindex-rs
WORKDIR /app
CMD ["/app/autoindex-rs"]
