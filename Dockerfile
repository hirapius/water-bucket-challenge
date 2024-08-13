FROM rust:1.80

WORKDIR /usr/src/water-bucket-challenge

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/water-bucket-challenge"]
