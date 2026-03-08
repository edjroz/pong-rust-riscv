.PHONY: build run clean

build:
	cargo build --release

run:
	cargo run --release

clean:
	cargo clean
