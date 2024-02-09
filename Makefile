all: build copy

build:
	cargo build --release

copy:
	cp target/release/ed25519_keygen ed25519-keygen

clean:
	cargo clean
	rm ed25519-keygen