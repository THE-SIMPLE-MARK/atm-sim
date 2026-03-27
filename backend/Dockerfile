FROM rust:latest as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY shared ./shared
COPY backend ./backend
# cargo complains if we don't have all workspace members present, even if it's not required for building
COPY app/src-tauri ./app/src-tauri 

RUN cargo build --release -p backend

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
  ca-certificates \
  libssl3 \
	libpq5
RUN rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/backend .

EXPOSE 3000

CMD ["./backend"]