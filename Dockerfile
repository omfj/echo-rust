FROM node:20-slim AS node_builder
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm install

FROM node:20-slim AS chef
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    pkg-config \
    libssl-dev \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable && \
    export PATH="/root/.cargo/bin:${PATH}" && \
    cargo install cargo-chef
ENV PATH="/root/.cargo/bin:${PATH}"

FROM chef AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
COPY --from=node_builder /app/node_modules ./node_modules
RUN cargo build --release --bin echo-rust

FROM node:20-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/echo-rust /usr/local/bin
COPY --from=builder /app/static ./static
COPY --from=builder /app/templates ./templates

ENTRYPOINT ["/usr/local/bin/echo-rust"]