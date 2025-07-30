FROM rust:latest as builder

WORKDIR /app

COPY ../../Cargo.toml ../../Cargo.lock ./

COPY ../../shared ./shared
COPY ../../auth ./auth
COPY ../../.sqlx ./.sqlx

RUN cargo build --release --package auth

FROM rust:slim AS stage-1

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/auth /usr/local/bin/auth

CMD ["auth"]
