use clap::{arg, command, value_parser};
use ed25519::lib::keygen::generate_key_pair;
use ed25519::lib::Key;
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
    let matches = command!()
        .arg(
            arg!([PREFIX] "prefix used in the key files filenames")
                .required(true)
                .value_parser(value_parser!(String)),
        )
        .get_matches();

    let prefix = matches.get_one::<String>("PREFIX").unwrap();

    let (public, private) = generate_key_pair();

    for (filename, key) in [
        (format!("{prefix}.sk"), private),
        (format!("{prefix}.pk"), public),
    ] {
        write_key(&filename, key).unwrap_or_else(|_| {
            eprintln!("Failed writing key in {filename}");
            std::process::exit(1)
        });
    }
}
