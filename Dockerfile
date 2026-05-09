# --- Stage 1: Builder ---
FROM rust:1.93-slim AS builder

RUN apt-get update && \
    apt-get install -y libpq-dev build-essential && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Build the actual app
COPY . .
RUN cargo build --release --all-features

# Build diesel_cli here
RUN cargo install diesel_cli --no-default-features --features postgres --version 2.2.12

# --- Stage 2: Runner ---
FROM debian:bookworm-slim AS runner

# Install ONLY the runtime postgres library (libpq5)
RUN apt-get update && \
    apt-get install -y libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 1. Copy your app binary
COPY --from=builder /app/target/release/inception-axum .

# 2. Copy the diesel binary from the builder's cargo bin
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# 3. Copy config/migration files
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/diesel.toml .

EXPOSE 8000

# You can now run migrations before starting the app in your entrypoint if needed
ENTRYPOINT ["./delifunds-axum"]
