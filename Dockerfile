# --------------+------------------------------------------------------------------------------------------------------------------------+
# BUILDER PHASE |                                                                                                                        |
# --------------+                                                                                                                        |
#                                                                                                                                        |
# PHASE: Builder phase                                                                                                                   |
# ---------------------------------------------------------------------------------------------------------------------------------------+

FROM alpine:latest AS builder

RUN apk update && apk add --no-cache \
    build-base \
    curl \
    git \
    openssl \
    ca-certificates \
    bash \
    libgcc

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustc --version

RUN curl -fsSL https://bun.sh/install | bash

ENV PATH="/root/.bun/bin:${PATH}"

RUN bun --version

WORKDIR /app

COPY . .

RUN cargo build --release

# -------------+-------------------------------------------------------------------------------------------------------------------------+
# RUNNER PHASE |                                                                                                                         |
# -------------+                                                                                                                         |
#                                                                                                                                        |
# PHASE: Runtime phase                                                                                                                   |
# ---------------------------------------------------------------------------------------------------------------------------------------+

FROM alpine:latest

RUN apk update && apk add --no-cache \
    openssl \
    ca-certificates \
    libgcc

WORKDIR /vilchain

COPY --from=builder /root/.bun /root/.bun
ENV PATH="/root/.bun/bin:${PATH}"

COPY --from=builder /chain/target/release/node .

CMD ["node"]

EXPOSE 8000