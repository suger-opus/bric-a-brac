###
# Build image
###
FROM rust:1.93 AS builder

WORKDIR /app

# Install protobuf compiler
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

# Copy workspace manifests (needed for workspace dependencies)
COPY Cargo.toml Cargo.lock ./

# Copy all workspace member Cargo.tomls (Cargo needs to see all members)
COPY crates/bric-a-brac-id/Cargo.toml crates/bric-a-brac-id/
COPY crates/bric-a-brac-protos/Cargo.toml crates/bric-a-brac-protos/
COPY ai/Cargo.toml ai/
COPY knowledge/Cargo.toml knowledge/
COPY metadata/Cargo.toml metadata/

# Create dummy microservices and crates to satisfy workspace (won't be built)
RUN mkdir -p knowledge/src && echo "fn main() {}" > knowledge/src/main.rs
RUN mkdir -p ai/src && echo "fn main() {}" > ai/src/main.rs

# Copy actual source code only for metadata and its dependencies
COPY crates/bric-a-brac-id/src crates/bric-a-brac-id/src
COPY crates/bric-a-brac-protos/src crates/bric-a-brac-protos/src
COPY metadata/src metadata/src
COPY metadata/migrations metadata/migrations
COPY metadata/.sqlx metadata/.sqlx

# Build only the metadata package
RUN cargo build --release --package metadata

###
# Runtime image
###
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/metadata /app/metadata

EXPOSE 8080

CMD ["/app/metadata"]
