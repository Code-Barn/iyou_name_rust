# Dockerfile for iyou_chart_kernel Rust extension
# Multi-stage build for development and production

# Argument for build configuration
ARG RUST_VERSION=stable
ARG PYTHON_VERSION=3.13
ARG BUILD_TYPE=release

# Stage 1: Builder
FROM rust:${RUST_VERSION}-slim as builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libmagickwand-dev \
    python3 \
    python3-dev \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Set up Python environment
ENV PYTHON_SYS_EXECUTABLE=/usr/bin/python3
ENV PYTHON_VERSION=${PYTHON_VERSION}

# Create and set up working directory
WORKDIR /app
COPY . .

# Build the Rust extension
RUN cargo build --${BUILD_TYPE}

# Stage 2: Runtime
FROM python:${PYTHON_VERSION}-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libmagickwand-7.q16-6 \
    && rm -rf /var/lib/apt/lists/*

# Create working directory
WORKDIR /app

# Copy built extension from builder
COPY --from=builder /app/target/${BUILD_TYPE}/libiyou_chart_kernel.so /app/

# Copy additional files
COPY README.md LICENSE /app/

# Set environment variables
ENV RUST_EXTENSION_PATH=/app/libiyou_chart_kernel.so
ENV PYTHONPATH=/app:${PYTHONPATH}

# Health check
HEALTHCHECK --interval=30s --timeout=3s \
  CMD python3 -c "import sys; sys.exit(0 if __import__('os').path.exists('/app/libiyou_chart_kernel.so') else 1)" || exit 1

# Default command (can be overridden)
CMD ["python3", "-c", "print('Rust extension container ready')"]
