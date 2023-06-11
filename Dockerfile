FROM rust as builder
WORKDIR /usr/src
COPY ./Cargo.toml .
COPY ./src/lib ./src/lib
RUN echo "fn main() {}\n" > dummy.rs
RUN sed -i 's/src\/main.rs/dummy.rs/' Cargo.toml
RUN cargo build --release
COPY . .
RUN cargo build --release --offline

FROM debian:buster-slim
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/src/self_signed_certs/ /usr/src/self_signed_certs/
COPY --from=builder /usr/src/target/release/server /usr/local/bin/server
EXPOSE 3000

CMD ["server"]
