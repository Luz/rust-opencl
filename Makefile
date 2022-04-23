run: all
	cargo run
clean:
	cargo clean
all: src/main.rs src/kernel.cl
	cargo build
