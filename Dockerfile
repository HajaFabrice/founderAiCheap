FROM rust:1.94-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY docs ./docs
COPY documents/99_Agent_Ready ./documents/99_Agent_Ready
COPY founder-brain ./founder-brain
COPY config ./config
COPY scripts ./scripts

RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates bash \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/founderai-ollama-rust /app/founderai-ollama-rust
COPY docs /app/docs
COPY documents/99_Agent_Ready /app/documents/99_Agent_Ready
COPY founder-brain /app/founder-brain
COPY config /app/config
COPY scripts /app/scripts

RUN mkdir -p /app/inbox /app/outbox /app/runtime /app/docs /app/documents \
    && chmod +x /app/scripts/*.sh

CMD ["/app/founderai-ollama-rust", "daemon", "--config", "/app/config/founderai.json"]
