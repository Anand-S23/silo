FROM rust:latest

WORKDIR /usr/src/server
COPY . .

RUN cargo build --release

CMD ["./target/release/silo"]
