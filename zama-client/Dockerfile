# Use a Rust base image
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy the entire project directory into the container
COPY . .

# Build your Rust project
RUN cargo build --release

# Set the entry point for the container
ENTRYPOINT ["target/release/zama-client"]
