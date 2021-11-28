all:
	cargo build

clean:
	cargo clean

lint:
	cargo clean
	cargo clippy

test:
	cargo test
