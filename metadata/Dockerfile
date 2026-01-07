###
# Build image
###
FROM rust:1.84 AS builder

WORKDIR /app

# Install protobuf compiler
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*

# Copy proto crate (shared dependency)
COPY crates/bric-a-brac-protos ./crates/bric-a-brac-protos

COPY metadata/Cargo.toml metadata/Cargo.lock ./metadata/
COPY metadata/src ./metadata/src

WORKDIR /app/metadata
RUN cargo build --release

###
# Runtime image
###
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/metadata/target/release/metadata /app/metadata

EXPOSE 8080

CMD ["/app/metadata"]
