FROM rust:1.51 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /usr/local/bin
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/random-load /usr/local/bin/random-load 
CMD ["random-load", "-c", "/etc/random-load/config.yaml"]