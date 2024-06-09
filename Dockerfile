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

WORKDIR /vilchain

COPY /chain .

WORKDIR /vilchain/chain/node
RUN cargo build --release

WORKDIR /vilchain/chain/manager/backend
RUN cargo build --release

WORKDIR /vilchain/chain/manager/frontend
COPY /chain/manager/frontend/package.json /chain/manager/frontend/bun.lockb ./
RUN bun install
RUN bun run build:app

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

COPY --from=builder /vilchain/chain/node/target/release/node /vilchain/node/

COPY --from=builder /vilchain/chain/manager/backend/target/release/backend /vilchain/admin/

COPY --from=builder /vilchain/chain/manager/frontend/build /vilchain/admin/build

COPY --from=builder /root/.bun /root/.bun
ENV PATH="/root/.bun/bin:${PATH}"

EXPOSE 8000 8080 3000

CMD ["sh", "-c", "/vilchain/node/node & /vilchain/admin/backend & bun run --port 3000 --prefix /vilchain/admin/build start"]

