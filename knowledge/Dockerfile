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
COPY crates/bric-a-brac-dtos/Cargo.toml crates/bric-a-brac-dtos/
COPY crates/bric-a-brac-id/Cargo.toml crates/bric-a-brac-id/
COPY crates/bric-a-brac-protos/Cargo.toml crates/bric-a-brac-protos/
COPY ai/Cargo.toml ai/
COPY knowledge/Cargo.toml knowledge/
COPY metadata/Cargo.toml metadata/

# Create dummy microservices and crates to satisfy workspace (won't be built)
RUN mkdir -p metadata/src && echo "fn main() {}" > metadata/src/main.rs
RUN mkdir -p ai/src && echo "fn main() {}" > ai/src/main.rs

# Copy actual source code only for knowledge and its dependencies
COPY crates/bric-a-brac-dtos/src crates/bric-a-brac-dtos/src
COPY crates/bric-a-brac-id/src crates/bric-a-brac-id/src
COPY crates/bric-a-brac-protos/build.rs crates/bric-a-brac-protos/
COPY crates/bric-a-brac-protos/protos crates/bric-a-brac-protos/protos
COPY crates/bric-a-brac-protos/src crates/bric-a-brac-protos/src
COPY knowledge/src knowledge/src

# Build only the knowledge package
RUN cargo build --release --package knowledge

###
# Runtime image
###
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/knowledge /app/knowledge

CMD ["/app/knowledge"]
