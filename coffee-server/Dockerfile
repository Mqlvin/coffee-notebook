FROM rust:1.85 as build

WORKDIR /usr/src/app
COPY ./ ./

RUN touch coffee.json
RUN cargo build --release

CMD ["./target/release/coffee-server"]
