# Create the build container to compile the hello world program
FROM rust:1.60 as builder

ADD . /app
WORKDIR /app
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get upgrade --yes && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/adder /adder

CMD ["/adder"]

