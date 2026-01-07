###
# Build image
###
FROM rust:1.84 AS builder

WORKDIR /app

# Install protobuf compiler
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

# Copy proto crate (shared dependency)
COPY crates/bric-a-brac-protos ./crates/bric-a-brac-protos

COPY knowledge/Cargo.toml knowledge/Cargo.lock ./knowledge/
COPY knowledge/src ./knowledge/src

WORKDIR /app/knowledge
RUN cargo build --release

###
# Runtime image
###
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/knowledge/target/release/knowledge /app/knowledge

EXPOSE 50051

CMD ["/app/knowledge"]
