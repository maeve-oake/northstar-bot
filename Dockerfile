FROM rust:alpine AS builder
RUN apk add --no-cache build-base
WORKDIR /app

COPY . .
RUN cargo build --release --bin northstar-bot

FROM alpine
COPY --from=builder /app/target/release/northstar-bot /usr/local/bin/northstar-bot
USER nobody
VOLUME [ "/data" ]
WORKDIR /data
ENTRYPOINT ["/usr/local/bin/northstar-bot"]
