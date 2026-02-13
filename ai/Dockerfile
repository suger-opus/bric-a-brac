###
# Build image
###
FROM rust:1.93 AS builder

WORKDIR /app

# Copy workspace manifests (needed for workspace dependencies)
COPY Cargo.toml Cargo.lock ./

# Copy all workspace member Cargo.tomls (Cargo needs to see all members)
COPY crates/bric-a-brac-id/Cargo.toml crates/bric-a-brac-id/
COPY crates/bric-a-brac-protos/Cargo.toml crates/bric-a-brac-protos/
COPY ai/Cargo.toml ai/
COPY knowledge/Cargo.toml knowledge/
COPY metadata/Cargo.toml metadata/

# Create dummy microservices and crates to satisfy workspace (won't be built)
RUN mkdir -p crates/bric-a-brac-id/src && echo "// ..." > crates/bric-a-brac-id/src/lib.rs
RUN mkdir -p crates/bric-a-brac-protos/src && echo "// ..." > crates/bric-a-brac-protos/src/lib.rs
RUN mkdir -p knowledge/src && echo "fn main() {}" > knowledge/src/main.rs
RUN mkdir -p metadata/src && echo "fn main() {}" > metadata/src/main.rs

# Copy actual source code only for ai and its dependencies
COPY ai/src ai/src

# Build only the ai package
RUN cargo build --release --package ai

###
# Runtime image
###
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/ai /app/ai

EXPOSE 8081

CMD ["/app/ai"]
