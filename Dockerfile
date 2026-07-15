FROM rust:1.88-alpine AS builder

WORKDIR /build
RUN apk add --no-cache musl-dev
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --locked --release

FROM alpine:3.24.1

RUN apk add --no-cache ca-certificates openssh-client-default \
    && addgroup -S shellcd \
    && adduser -S -D -H -G shellcd shellcd
COPY --from=builder /build/target/release/shellcd-basic /usr/local/bin/shellcd-basic

USER shellcd:shellcd
ENTRYPOINT ["/usr/local/bin/shellcd-basic"]
