FROM rust:1.62 as builder
WORKDIR /usr/src/hanebo
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update \
    && apt-get install -y \
        libsqlite3-dev \
        libssl-dev \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/hanebo /usr/local/bin/hanebo
CMD ["hanebo"]
