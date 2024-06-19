# Use the official Rust image as a build environment
FROM rust:latest AS builder

# Set the working directory inside the container

# Copy the Cargo.toml and Cargo.lock files

# Copy the source code
COPY apps/backend/Cargo.toml apps/backend/Cargo.toml
COPY apps/backend/src apps/backend/src
COPY apps/backend/Cargo.lock apps/backend/Cargo.lock

COPY packages/db/client/Cargo.toml packages/db/client/Cargo.toml
COPY packages/db/client/src packages/db/client/src
COPY packages/db/client/Cargo.lock packages/db/client/Cargo.lock

COPY packages/db/migration/Cargo.toml packages/db/migration/Cargo.toml
COPY packages/db/migration/src packages/db/migration/src
COPY packages/db/migration/Cargo.lock packages/db/migration/Cargo.lock

# Build the project
RUN printenv
RUN cd apps/backend && cargo build --release

# Expose the port that the Actix server will run on
EXPOSE 8080


# Set the entry point to run the Actix server
CMD ["./apps/backend/target/release/backend"]