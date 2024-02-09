use ed25519::lib::ed25519_keygen::{generate_key_pair, Key};
use std::fs;
use std::io::Write;

fn write_key(filename: &str, key: Key) -> Result<(), std::io::Error> {
    let mut key_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(filename)?;

    key_file.write_all(&key)?;
    Ok(())
}

fn main() {
    let (public, private) = generate_key_pair();

    for (filename, key) in [("prefix.sk", private), ("prefix.pk", public)] {
        write_key(filename, key).unwrap_or_else(|_| {
            eprintln!("Failed writing key in {filename}");
            std::process::exit(1)
        });
    }
}
