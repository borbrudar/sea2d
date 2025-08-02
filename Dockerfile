# Stage 1: Build the Rust application with SDL2 libraries
FROM debian:bullseye as builder

# Install required tools and SDL2 dev packages
RUN apt-get update && apt-get install -y \
    curl build-essential pkg-config cmake git \
    libsdl2-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-ttf-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust using rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

# Copy manifests and cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "// dummy" > src/lib.rs
RUN cargo build --release || true

# Copy actual source code
COPY . .

# Build the actual binary
RUN cargo build --release

# Stage 2: Runtime image (optional, strip everything down)
FROM debian:bullseye-slim

# Install only runtime SDL2 libraries
RUN apt-get update && apt-get install -y \
    libsdl2-2.0-0 libsdl2-image-2.0-0 libsdl2-mixer-2.0-0 libsdl2-ttf-2.0-0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/sea2d ./sea2d
COPY --from=builder /app/resources ./resources

ENTRYPOINT ["./sea2d"]
