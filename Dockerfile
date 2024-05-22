# ---------------------------------------------------------------------------------------------------------------
# BUILDER STAGE
# ---------------------------------------------------------------------------------------------------------------
    FROM alpine:latest AS builder

    RUN apk update && \
    apk add --no-cache \
    build-base \
    openssl-dev \
    curl 
    
    # Install Rust
    RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    source $HOME/.cargo/env && \
    rustup install stable && \
    rustup default stable
    
    WORKDIR /app
    
    COPY ./chain .
    
    # Configure environment to use cargo
    ENV PATH="/root/.cargo/bin:${PATH}"
    
    RUN cargo build --release
    
    # ---------------------------------------------------------------------------------------------------------------
    # FINAL STAGE
    # ---------------------------------------------------------------------------------------------------------------
    FROM alpine:latest
    
    RUN apk update 
    
    WORKDIR /vilchain
    
    COPY --from=builder /app/target/release/node .
    
    CMD ["./node"]