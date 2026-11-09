FROM rust:1.99-bookworm AS build
WORKDIR /app
COPY . .
RUN cargo build --release --bin saptrans-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=build /app/target/release/saptrans-server /usr/local/bin/saptrans-server
EXPOSE 3000
CMD ["saptrans-server"]
