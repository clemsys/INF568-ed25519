all: build copy

build:
	cargo build --release

copy:
	cp target/release/{keygen,sign,verify} .

clean:
	cargo clean
	rm {keygen,sign,verify}