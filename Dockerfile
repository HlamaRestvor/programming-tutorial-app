# ==============================================================================
# Ferris Garden - Multi-stage Production Dockerfile
# Stage 1: Build binary with cargo dependency caching
# Stage 2: Minimal, secure Debian runtime with ca-certificates and curl healthcheck
# ==============================================================================

# ------------------------------------------------------------------------------
# 1. Builder Stage
# ------------------------------------------------------------------------------
FROM rust:1-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies required for compiling C/OpenSSL bindings
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Cache dependency layer: build dependencies first with dummy main.rs
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy real sources and build the final release binary
COPY src ./src
RUN touch src/main.rs && cargo build --release

# ------------------------------------------------------------------------------
# 2. Production Runner Stage
# ------------------------------------------------------------------------------
FROM debian:bookworm-slim AS runner

WORKDIR /app

# Install ca-certificates (for HTTPS / Brevo API) and curl (for healthchecks)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy compiled binary from builder
COPY --from=builder /app/target/release/programming-tutorial-app /app/programming-tutorial-app

# Copy runtime assets: Tera HTML templates and static assets (CSS, JS)
COPY templates /app/templates
COPY static /app/static

# Create persistent storage directory for SQLite database
RUN mkdir -p /data

# Default runtime configuration (overridable via container env)
ENV DATABASE_URL=sqlite:///data/app.db?mode=rwc \
    RUST_LOG=info,programming_tutorial_app=debug,tower_http=info

EXPOSE 3000

# Container liveness health check targeting the Axum /health route
HEALTHCHECK --interval=20s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:${PORT}/health || exit 1

# Start Axum application server
CMD ["/app/programming-tutorial-app"]
