# Multi-stage build for Leptos app
FROM rust:1.89 as builder

# Install cargo-leptos
RUN cargo install cargo-leptos

# Install Node.js for build dependencies
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs

# Set working directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./
COPY package.json ./

# Install Node dependencies
RUN npm install

# Copy source code
COPY . .

# Build the application
RUN cargo leptos build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1001 app

# Copy the binary and site directory
COPY --from=builder /app/target/release/ricochet-docs /usr/local/bin/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/public /app/public

# Set ownership
RUN chown -R app:app /app

# Switch to app user
USER app

# Set working directory
WORKDIR /app

# Expose port
EXPOSE 8080

# Run the application
CMD ["ricochet-docs"]
