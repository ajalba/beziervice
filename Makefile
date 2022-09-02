build:
	RUSTFLAGS="$RUSTFLAGS -A warnings" cargo build --manifest-path ./Beziervice/Cargo.toml
install:
	RUSTFLAGS="$RUSTFLAGS -A warnings" cargo build --release --manifest-path ./Beziervice/Cargo.toml
test:
	RUSTFLAGS="$RUSTFLAGS -A warnings" cargo build --manifest-path ./Beziervice/Cargo.toml
	RUSTFLAGS="$RUSTFLAGS -A warnings" cargo test --manifest-path ./Beziervice/Cargo.toml
run:
	cargo build --release --manifest-path ./Beziervice/Cargo.toml
	cargo run --manifest-path ./Beziervice/Cargo.toml
