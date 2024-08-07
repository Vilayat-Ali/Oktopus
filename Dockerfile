# =================================================================================================
# BUILDER STAGE
# =================================================================================================

FROM alpine:latest AS BUILDER

RUN apk update && apk add --no-cache \
    bash \
    curl \
    gcc \
    g++ \
    make \
    musl-dev \
    openssl-dev \
    perl

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustc --version && cargo --version

WORKDIR /oktopus

COPY ./chain /oktopus/

RUN cargo build --release

# =================================================================================================
# RELEASE STAGE
# =================================================================================================

FROM alpine:latest AS RELEASE

WORKDIR /oktopus

COPY --from=BUILDER /oktopus/target/release/node .

CMD ["./node"]

EXPOSE 8000