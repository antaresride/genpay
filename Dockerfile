# --- STAGE 1: Compiler (Builder) ---
FROM rust:bookworm AS compiler
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl lsb-release software-properties-common gnupg git libz-dev libedit-dev libxml2-dev \
    && rm -rf /var/lib/apt/lists/*

# Install LLVM 21 using curl
RUN curl -fsSL https://apt.llvm.org/llvm.sh -o llvm.sh && chmod +x llvm.sh && ./llvm.sh 21 all && rm llvm.sh

ENV LLVM_SYS_211_PREFIX=/usr/lib/llvm-21
ENV LIBCLANG_PATH=/usr/lib/llvm-21/lib
ENV PATH="/usr/lib/llvm-21/bin:${PATH}"

# Build the project
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# --- STAGE 2: Development ---
FROM compiler AS dev
WORKDIR /workspaces/genpay
ENTRYPOINT ["/bin/bash"]

# --- STAGE 3: Runtime ---
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    libedit2 libz1 libxml2 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=compiler /app/target/release/genpay /usr/local/bin/genpay
WORKDIR /src
ENTRYPOINT ["genpay"]
