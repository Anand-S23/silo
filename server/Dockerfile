FROM rust:1.68

WORKDIR /usr/src/server
COPY . .

RUN cargo build --release

CMD ["./target/release/silo"]