FROM rust:1.86.0-slim-bullseye

WORKDIR /app

COPY Cargo.toml ./

COPY src/* .
COPY static/* static/

RUN cargo build --release

EXPOSE 8080

CMD ["cargo", "run"]