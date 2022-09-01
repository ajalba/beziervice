build:
	cargo build --manifest-path ./Beziervice/Cargo.toml
test:
	cargo test --manifest-path ./Beziervice/Cargo.toml
run:
	cargo run --manifest-path ./Beziervice/Cargo.toml
install:
	cargo build --release --manifest-path ./Beziervice/Cargo.toml