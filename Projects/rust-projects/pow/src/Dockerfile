# Use an official Rust image as a build stage
FROM rust:latest AS builder

# Set the working directory
WORKDIR /usr/local

# Copy the Cargo.toml and Cargo.lock to the container
COPY . .

# Build the dependencies
RUN cargo build --release

# Use a smaller, Ubuntu-based production image to run the app
FROM ubuntu

# Set the working directory for the application
WORKDIR /usr/local

# Install required dependencies for the Rust application (e.g., libc, SSL libraries)
RUN apt-get update && apt-get install -y \
    libssl-dev 
# Copy the compiled Rust binary from the builder image
COPY --from=builder /usr/local/target/release/pow  ./pow

# Set the entrypoint for the container
CMD ["./pow"]