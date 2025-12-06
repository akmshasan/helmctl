# syntax=docker/dockerfile:1
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/helmctl /usr/local/bin/helmctl
ENTRYPOINT ["helmctl"]
CMD ["--help"]
