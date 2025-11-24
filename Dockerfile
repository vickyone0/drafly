#############################################
# 1. CARGO CHEF — Dependency caching
#############################################
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Copy project files
COPY . .

# Create recipe file to cache dependencies
RUN cargo chef prepare --recipe-path recipe.json



#############################################
# 2. BUILDER — Build dependencies + app
#############################################
FROM lukemathwalker/cargo-chef:latest-rust-1 AS builder
WORKDIR /app

# Install system dependencies for Rust TLS/reqwest/openssl
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Enable SQLx OFFLINE MODE
ENV SQLX_OFFLINE=true

# Build dependencies using cached recipe
COPY --from=chef /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source and build the app
COPY . .
RUN cargo build --release



#############################################
# 3. RUNTIME — Minimal production image
#############################################
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Install SSL libraries
RUN apt-get update && apt-get install -y libssl3 ca-certificates && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Copy compiled binary
COPY --from=builder /app/target/release/drafly /app/drafly

# Expose app port
EXPOSE 8000

# Start server
CMD ["./drafly"]
