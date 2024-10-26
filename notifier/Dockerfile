FROM rust:1.78-slim-bookworm AS builder
WORKDIR /app

ARG DATABASE_URL
ENV DATABASE_URL=$(DATABASE_URL)

COPY .. .
RUN cargo build --release --bin notifier

FROM debian:bookworm-slim
WORKDIR /app
RUN adduser manga-update-notifier && chwon -R manga-update-notifier /app
USER manga-update-notifier
COPY --from=builder ./app/target/release/notifier ./target/release/notifier

ENTRYPOINT ["./target/release/notifier"]