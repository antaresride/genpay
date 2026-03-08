# --- STAGE 1: Compiler  ---
FROM rust:bookworm AS compiler
RUN apt-get update && apt-get install -y --no-install-recommends \
    wget lsb-release software-properties-common gnupg curl git libz-dev libedit-dev libxml2-dev \
    && rm -rf /var/lib/apt/lists/*
RUN wget https://apt.llvm.org -O llvm.sh && chmod +x llvm.sh && ./llvm.sh 21 all && rm llvm.sh
ENV LLVM_SYS_211_PREFIX=/usr/lib/llvm-21
ENV LIBCLANG_PATH=/usr/lib/llvm-21/lib
ENV PATH="/usr/lib/llvm-21/bin:${PATH}"

# --- STAGE 2: Development ---
# This inherits everything from 'compiler'
FROM compiler AS dev
WORKDIR /workspaces/genpay
# We stay as 'dev' has all tools available
ENTRYPOINT ["/bin/bash"]

# --- STAGE 3: Runtime ---
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    libedit2 libz1 libxml2 \
    && rm -rf /var/lib/apt/lists/*
# We go back to 'compiler' to grab only the finished binary
COPY --from=compiler /app/target/release/genpay /usr/local/bin/genpay
WORKDIR /src
ENTRYPOINT ["genpay"]
