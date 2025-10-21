
FROM rust:1.85 as builder

WORKDIR /app

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only (for better caching)
RUN cargo build --release

# Remove dummy binary
RUN rm -f target/release/deps/rust_indexer*

# Copy actual source code
COPY . .

# Build the real application
RUN cargo build --release

# Strip binary to reduce size
RUN strip target/release/rust_indexer

# Runtime stage - distroless (minimal Google image)
FROM gcr.io/distroless/cc-debian12

# Copy the binary
COPY --from=builder /app/target/release/rust_indexer /rust_indexer

CMD ["/rust_indexer"]

# FROM rust:1.85 as builder
#
# WORKDIR /app
#
# # Copy manifest files
# COPY Cargo.toml Cargo.lock ./
#
# # Create dummy main.rs to build dependencies
# RUN mkdir src && echo "fn main() {}" > src/main.rs
#
# # Build dependencies with musl target
# RUN rustup target add x86_64-unknown-linux-musl
# RUN cargo build --target x86_64-unknown-linux-musl --release
#
# # Remove dummy binary
# RUN rm -f target/x86_64-unknown-linux-musl/release/deps/rust_indexer*
#
# # Copy actual source code
# COPY . .
#
# # Build the real application
# RUN cargo build --target x86_64-unknown-linux-musl --release
#
# # Strip binary
# RUN strip target/x86_64-unknown-linux-musl/release/rust_indexer
#
# # Runtime stage
# FROM scratch
#
# # Copy the binary
# COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust_indexer /rust_indexer
#
# CMD ["/rust_indexer"]
# # Use cargo-chef for better dependency caching
# FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
# WORKDIR /app
#
# FROM chef AS planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json
#
# FROM chef AS builder
# # Install musl-tools BEFORE trying to compile for musl
# RUN apt-get update && apt-get install -y \
#   musl-tools \
#   pkg-config \
#   && rm -rf /var/lib/apt/lists/*
#
# RUN rustup target add x86_64-unknown-linux-musl
#
# # Copy recipe
# COPY --from=planner /app/recipe.json recipe.json
#
# # Copy path dependencies explicitly so that `cargo chef cook` works
# # COPY entity ./entity
# # COPY migration ./migration
# # Cook dependencies
# RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
#
# # Now copy full source
# COPY . .
#
# # Build app
# RUN cargo build --release --target x86_64-unknown-linux-musl
#
# # Runtime stage - ultra-small, scratch
# FROM scratch AS runtime
#
# # Copy CA certificates (essential for HTTPS)
# COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
#
# # Copy the *musl-compiled* binary
# COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust_indexer /rust_indexer
#
# # EXPOSE 3000
#
# # Run the binary from the root directory
# CMD ["/rust_indexer"]
