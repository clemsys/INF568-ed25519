use clap::{arg, command, value_parser};
use ed25519::lib::verify::verify;
use ed25519::lib::{Key, Signature};
use std::fs;

fn main() {
    let matches = command!()
        .arg(
            arg!([PKFILE] ".pk file containing the public key")
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

    let pkfile = matches.get_one::<String>("PKFILE").unwrap();
    let datafile = matches.get_one::<String>("DATAFILE").unwrap();
    let sigfile = matches.get_one::<String>("SIGFILE").unwrap();

    let public: Key = fs::read(pkfile)
        .unwrap_or_else(|_| {
            eprintln!("Failed reading {pkfile}");
            std::process::exit(1)
        })
        .try_into()
        .unwrap_or_else(|_| {
            eprintln!("Invalid key in {pkfile}");
            std::process::exit(1)
        });

    let message = fs::read(datafile).unwrap_or_else(|_| {
        eprintln!("Failed reading {datafile}");
        std::process::exit(1)
    });

    let signature: Signature = fs::read(sigfile)
        .unwrap_or_else(|_| {
            eprintln!("Failed reading {sigfile}");
            std::process::exit(1)
        })
        .try_into()
        .unwrap_or_else(|_| {
            eprintln!("Invalid signature in {sigfile}");
            std::process::exit(1)
        });

    if verify(public, &message, signature) {
        println!("ACCEPT");
    } else {
        println!("REJECT");
    }
}
