use clap::{arg, command, value_parser};
use ed25519::lib::sign::sign;
use ed25519::lib::Key;
use std::fs;
use std::io::Write;

fn main() {
    let matches = command!()
        .arg(
            arg!([PREFIX] "prefix used in {prefix}.sk and {prefix}.pk")
                .required(true)
                .value_parser(value_parser!(String)),
        )
        .arg(
            arg!([DATAFILE] "file containing the message to sign")
                .required(true)
                .value_parser(value_parser!(String)),
        )
        .arg(
            arg!([SIGFILE] "file where to write the signature")
                .required(true)
                .value_parser(value_parser!(String)),
        )
        .get_matches();

    let prefix = matches.get_one::<String>("PREFIX").unwrap();
    let datafile = matches.get_one::<String>("DATAFILE").unwrap();
    let sigfilename = matches.get_one::<String>("SIGFILE").unwrap();

    let private: Key = fs::read(format!("{prefix}.sk"))
        .unwrap_or_else(|_| {
            eprintln!("Failed reading {prefix}.sk");
            std::process::exit(1)
        })
        .try_into()
        .unwrap_or_else(|_| {
            eprintln!("Invalid key in {prefix}.sk");
            std::process::exit(1)
        });

    let message = fs::read(datafile).unwrap_or_else(|_| {
        eprintln!("Failed reading {datafile}");
        std::process::exit(1)
    });

    let mut sigfile = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(sigfilename)
        .unwrap_or_else(|_| {
            eprintln!("Failed opening {sigfilename}");
            std::process::exit(1)
        });

    let signature = sign(private, &message);

    sigfile.write_all(&signature).unwrap_or_else(|_| {
        eprintln!("Failed writing signature in {sigfilename}");
        std::process::exit(1)
    });
}
