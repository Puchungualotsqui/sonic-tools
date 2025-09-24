# --- build stage ---
FROM rust:1.89-bookworm AS build
WORKDIR /app

# protoc for tonic/prost codegen
RUN apt-get update && apt-get install -y --no-install-recommends \
    protobuf-compiler ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 1) Cache deps: copy only Cargo metadata first
COPY Services/rust-audio/Cargo.toml .
# Copy Cargo.lock if you have one (won't fail if missing)
COPY Services/rust-audio/Cargo.lock* ./

# Prime dependency build cache
RUN mkdir -p src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# 2) Copy real sources and proto
COPY Services/rust-audio/ ./
COPY proto/ ./proto

# 3) Build (build.rs will find ./proto)
RUN cargo build --release

# --- run stage ---
FROM debian:bookworm-slim
WORKDIR /app

# Install ffmpeg (includes ffprobe) + ca-certificates for TLS
RUN apt-get update && apt-get install -y --no-install-recommends \
    ffmpeg ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/rust-audio /usr/local/bin/rust-audio

EXPOSE 50051
ENTRYPOINT ["rust-audio"]
