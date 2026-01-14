# A multi-stage container file for the pubd application.

#
# Build stage
#

FROM rust:1.91-alpine3.23 AS builder

RUN apk add --no-cache musl-dev protobuf-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./

COPY proto ./proto

COPY build.rs ./

COPY src ./src

RUN cargo build --release

#
# Run stage
#

FROM alpine:latest

RUN apk add --no-cache libgcc

WORKDIR /app

COPY --from=builder /app/target/release/pubd /app/pubd

EXPOSE 60069

CMD ["/app/pubd"]
