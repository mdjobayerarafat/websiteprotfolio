# ==========================================
# Multi-stage Dockerfile for Rust Portfolio
# ==========================================

# Stage 1: Builder - Using Rust 1.85+ for edition 2024 support
FROM rustlang/rust:nightly-bookworm AS builder

# Install build dependencies for SQLite
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty project
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Create dummy src to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Build the actual application
RUN touch src/main.rs && \
    cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libsqlite3-0 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user for security
RUN useradd -r -s /bin/false appuser

# Create app directory
WORKDIR /app

# Create directories for data and static files
RUN mkdir -p /app/data /app/static /app/templates && \
    chown -R appuser:appuser /app

# Copy the binary from builder
COPY --from=builder /app/target/release/portfolio /app/portfolio

# Copy static files and templates
COPY --chown=appuser:appuser static ./static
COPY --chown=appuser:appuser templates ./templates

# Set ownership of the binary
RUN chown appuser:appuser /app/portfolio

# Switch to non-root user
USER appuser

# Environment variables
ENV RUST_LOG=info
ENV DATABASE_URL=/app/data/portfolio.db

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/ || exit 1

# Run the application
CMD ["./portfolio"]
