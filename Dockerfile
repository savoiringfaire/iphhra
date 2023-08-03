FROM rust:1.67 as builder
WORKDIR /usr/src/iphhra
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/iphhra /usr/local/bin/iphhra
CMD ["iphhra"]
