# --- Stage 1: Build (Rust 1.94.0) ---
FROM rust:1.94-bookworm AS builder

ARG BRANCH=main

# 1. Install system dependencies
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    wget lsb-release software-properties-common gnupg curl git libz-dev libedit-dev libxml2-dev \
    && rm -rf /var/lib/apt/lists/*

# 2. Download and run llvm.sh (FIXED URL)
RUN wget https://apt.llvm.org/llvm.sh -O llvm.sh \
    && chmod +x llvm.sh \
    && ./llvm.sh 21 all \
    && rm llvm.sh \
    && ln -sf /usr/lib/llvm-21/bin/llvm-config /usr/bin/llvm-config

# 3. Set LLVM 21 environment variables for Rust (llvm-sys)
ENV LLVM_SYS_211_PREFIX=/usr/lib/llvm-21
ENV LIBCLANG_PATH=/usr/lib/llvm-21/lib
ENV PATH="/usr/lib/llvm-21/bin:$PATH"

# 4. Clone your GitHub repository
WORKDIR /app
RUN git clone --depth 1 -b ${BRANCH} https://github.com/antaresride/genpay.git .

# 5. Build the release binary
RUN cargo build --release --bin genpay

# --- Stage 2: Runtime (Slim) ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    libedit2 libz1 libxml2 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/genpay /usr/local/bin/genpay
RUN mkdir /src
WORKDIR /src
ENTRYPOINT ["genpay"]
