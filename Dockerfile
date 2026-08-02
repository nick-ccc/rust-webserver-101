FROM lukemathwalker/cargo-chef:latest-rust-1.94.0 AS chef
WORKDIR /app
RUN apt-get update && \
    apt-get install -y \
        clang \
        lld \
        musl-tools \
        musl-dev \
        ca-certificates && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*

FROM chef AS planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder

RUN rustup target add x86_64-unknown-linux-musl
COPY --from=planner /app/recipe.json recipe.json

# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo chef cook \
  --release \
  --target x86_64-unknown-linux-musl \
  --recipe-path recipe.json

COPY . .
ENV SQLX_OFFLINE true
# Build our project
RUN cargo build \
  --release \
  --target x86_64-unknown-linux-musl \
  --bin trsws

# This ends up around ~10MiB using scratch!
FROM scratch AS runtime
WORKDIR /app

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/trsws trsws
COPY configuration configuration

ENV APP_ENVIRONMENT production

ENTRYPOINT ["./trsws"]
