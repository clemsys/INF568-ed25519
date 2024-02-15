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

Edwards and Montgomery arithmetic are implemented in `src/lib/arithmetic/`. Each file corresponds to a specific type of point (`ProjEdPoint`, `ProjMPoint`, `MPoint`, `XLineProjMPoint`), with relevant operations and conversions to the other types.

Files in `src/bin/` are only here to produce the binaries, so they mostly contain a main function, which calls functions from `src/lib/` directly.

## Extensions

The implementation from the main branch is not completely compliant with the RFC because some extensions have been implemented. For a fully compliant version, use the branch `rfc-compliant`.

### Constant time scalar multiplication

Constant time scalar multiplication is achieved by converting Edwards coordinates to Montgomery coordinates to use the (constant-time) Montgomery-Ladder for scalar multiplication. The Montgomery-Ladder outputs an x-line point, so y has to be recovered thanks to the _Okeya–Sakurai y-coordinate recovery algorithm_, before the Montgomery point can be converted back to Edwards coordinates.
