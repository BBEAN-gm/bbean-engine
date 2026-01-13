FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release -p bbean-cli

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/bbean-cli /usr/local/bin/bbean
COPY config.json /etc/bbean/config.json

ENV BBEAN_CONFIG=/etc/bbean/config.json
EXPOSE 9420

ENTRYPOINT ["bbean", "start"]
