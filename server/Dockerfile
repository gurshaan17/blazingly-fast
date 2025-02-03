# Use the latest Rust image as the base
FROM rust:latest

# Install required dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libpq-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Install rustup and set the Rust version to 1.84.1
RUN rustup install 1.84.1 && \
    rustup default 1.84.1

# Create and set working directory
WORKDIR /usr/src/app

# Copy the entire project
COPY . .

# Show directory contents for debugging
RUN ls -la

# Show Cargo.toml contents for debugging
RUN cat Cargo.toml

# Build the release binary
RUN cargo build --release --verbose

# Expose the port your application listens on
EXPOSE 8080

# Run the binary
CMD ["./target/release/blazingly-fast"]