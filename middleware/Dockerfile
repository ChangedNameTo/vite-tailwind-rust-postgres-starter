# middleware/Dockerfile
FROM rust:latest

WORKDIR /app

# Install diesel_cli
RUN apt-get update && apt-get install -y libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres

COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--release"]