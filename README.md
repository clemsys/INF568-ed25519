# INF568 Assignment 5 - ed25519

Author: [Clément CHAPOT](mailto:clement.chapot@polytechnique.edu) <br>
Description: implementation of ed25519 (see: [RFC 8032](https://datatracker.ietf.org/doc/html/rfc8032)) as part of INF568 course at École polytechnique

## Building

Build the project using `make`.

This calls `cargo build --release` and copies binaries from `target/release/` into the project root.

## Usage

TODO

For more precise usage information, use `--help` on the relevant binary.

## Testing

`cargo test` checks if the binaries and the intermediate functions produce the right output, by using the tests from the RFC.

## Project structure

The core of the project can be found in `src/lib/`.

`src/lib/montgomery.rs` provides functions to perform arithmetic on elliptic curves, which are used in `src/lib/x25519.rs` to implement Diffie-Hellman on the 25519 curve.

Files in `src/bin/` are here to produce the binaries, so they only contain a main functions, which call functions from `src/lib/` directly.
