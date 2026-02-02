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
COPY crates/bric-a-brac-protos/Cargo.toml ./crates/bric-a-brac-protos/
COPY crates/bric-a-brac-id/Cargo.toml ./crates/bric-a-brac-id/
COPY knowledge/Cargo.toml ./knowledge/

# Create dummy metadata manifest and lib to satisfy workspace (won't be built)
COPY metadata/Cargo.toml ./metadata/
RUN mkdir -p metadata/src && echo "// Dummy file for workspace" > metadata/src/lib.rs

# Copy actual source code only for knowledge and its dependencies
COPY crates/bric-a-brac-protos ./crates/bric-a-brac-protos
COPY crates/bric-a-brac-id ./crates/bric-a-brac-id
COPY knowledge/src ./knowledge/src

# Build only the knowledge package
RUN cargo build --release --package knowledge

###
# Runtime image
###
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/knowledge /app/knowledge

EXPOSE 50051

CMD ["/app/knowledge"]
