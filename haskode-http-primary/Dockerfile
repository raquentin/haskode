FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /app/target/release/haskode-http-primary usr/local/bin/haskode-http-primary

EXPOSE 3000

ENTRYPOINT ["haskode-http-primary"]
