FROM rust:1.80

WORKDIR /usr/src/water-bucket-challenge

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo fetch

RUN cargo install --path . --root /usr/local

EXPOSE 8080

CMD ["/usr/local/bin/water-bucket-challenge"]
