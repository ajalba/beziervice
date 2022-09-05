FROM rust:1.63

RUN apt-get update; apt-get install make;apt install -y libpq-dev;

COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres;

CMD bash -c "diesel setup && cargo run --manifest-path ./Beziervice/Cargo.toml"