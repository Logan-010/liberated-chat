# Ensure user port is set
ARG PORT=8080

# Build frontend
FROM rust:alpine AS builder

# Open directory
WORKDIR /app/building

# Copy source code
COPY . .

# Force to use system package rather than compiling new one
ENV OPENSSL_NO_VENDOR 1

# Install deps
RUN apk add --no-cache musl-dev gcc pkgconfig openssl-dev binutils

# Force OpenSSL to use alpine version
ENV OPENSSL_LIB_DIR=/usr/lib/
ENV OPENSSL_INCLUDE_DIR=/usr/include/

# Add target from wasm frontend
RUN rustup target add wasm32-unknown-unknown

# Install build tools
RUN cargo install just
RUN just install-dependencies

# Build bundle
RUN just bundle

# Run the app
FROM alpine:latest

# Open app root directory
WORKDIR /app

# Copy in bundle
COPY --from=builder /app/building/bundle /app/release

# Open bundle directory
WORKDIR /app/release

# Add an unprivileged user
RUN adduser -D appuser

# Change ownership of the app directory to the new user
RUN chown -R appuser /app/release

# Select user
USER appuser

# Open port for app
EXPOSE $PORT

# Start app
CMD ["./liberated-chat-server"]
