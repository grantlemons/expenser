FROM rust as builder
WORKDIR /app
COPY . .
RUN echo "fn main() {}\n" > dummy.rs
RUN sed -i 's/src\/main.rs/dummy.rs/' Cargo.toml
RUN cargo build --release
RUN cargo build --release --offline --bin server

FROM debian:buster-slim
RUN apt-get update && apt-get install -y curl libpq-dev && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /app/self_signed_certs/ /usr/src/self_signed_certs/
COPY --from=builder /app/target/release/server /usr/local/bin/server
EXPOSE 3000

CMD ["server"]
