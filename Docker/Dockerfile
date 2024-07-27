# Use the official Rust image as a build environment
FROM rust:latest AS builder

# Set the working directory
WORKDIR /lounge_server

# Copy the Cargo files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Copy the source files
COPY src ./src

# Build the project
RUN cargo build --release

# Use the same Rust base image for the runtime environment to ensure compatibility
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/lounge_server

# Copy the build artifact from the build stage
COPY --from=builder /lounge_server/target/release/lounge_server /usr/local/bin/lounge_server

# Expose the port the application runs on
EXPOSE 443

# Set the startup command to run the binary

