build:
	cargo build --manifest-path ./Beziervice/Cargo.toml
install:
	cargo build --release --manifest-path ./Beziervice/Cargo.toml
test:
	cargo build --manifest-path ./Beziervice/Cargo.toml
	cargo test --manifest-path ./Beziervice/Cargo.toml
run:
	cargo build --release --manifest-path ./Beziervice/Cargo.toml
	cargo run --manifest-path ./Beziervice/Cargo.toml
