# INF568 Assignment 5 - ed25519

Author: [Clément CHAPOT](mailto:clement.chapot@polytechnique.edu)<br>
Description: implementation of ed25519 (see: [RFC 8032](https://datatracker.ietf.org/doc/html/rfc8032)) as part of INF568 course at École polytechnique

## Building

Build the project using `make`.

This calls `cargo build --release` and copies the three binaries `keygen`, `sign` and `verify` from `target/release/` into the project root.

## Usage

- `./keygen <PREFIX>` generates a random secret key (stored in `{PREFIX}.sk`) and computes the corresponding public key (stored in `{PREFIX}.pk`)
- `./sign <PREFIX> <DATAFILE> <SIGFILE>` computes the signature of the message stored in `DATAFILE` given the secret key `{PREFIX}.sk`, and stores it in `SIGFILE`
- `./verify <PKFILE> <DATAFILE> <SIGFILE>` verifies that the signature stored in `SIGFILE` is valid for the message stored in `DATAFILE` given the public key `PKFILE`. It prints either `ACCEPT\n` or `REJECT\n`.

For more precise usage information, use `--help` on the relevant binary.

## Testing

`cargo test` checks if the binaries and the intermediate functions produce the right output, by using the test vectors from the RFC.

## Project structure

The core of the project can be found in `src/lib/`.

Files in `src/bin/` are only here to produce the binaries, so they mostly contain a main function, which calls functions from `src/lib/` directly.
