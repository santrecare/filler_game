FROM rust:1.75-buster as builder
WORKDIR /usr/src/app
COPY . .
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && \
    apt-get install -y libssl1.1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/arena /usr/local/bin/
CMD ["arena"]
