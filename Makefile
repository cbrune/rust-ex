all:
	cargo build

clean:
	cargo clean

lint:
	cargo clean
	cargo clippy -- -D warnings

test:
	cargo test
