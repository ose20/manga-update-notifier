FROM rust:1.78-slim-bookworm AS builder
WORKDIR /app

ARG DATABASE_URL
ENV DATABASE_URL=$(DATABASE_URL)

COPY .. .
RUN cargo build --release --bin batch

FROM debian:bookworm-slim
WORKDIR /app
RUN adduser batch && chwon -R batch /app
USER manga-update-notifier
COPY --from=builder ./app/target/release/batch ./target/release/batch

ENTRYPOINT ["./target/release/batch"]