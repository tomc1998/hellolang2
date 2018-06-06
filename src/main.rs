extern crate clap;
extern crate colored;

#[macro_use]
mod output;
mod token;
mod lex;

use std::io::{ErrorKind};
use clap::{App, Arg};
use colored::*;

fn main() {
    let matches = App::new("hl2 compiler")
        .version("0.1")
        .about("Compiles .hl2 files")
        .arg(Arg::with_name("input-file")
                .help("The file to compile")
                .index(1).required(true))
        .get_matches();
    let filename = matches.args.get("input-file").unwrap().vals[0].clone().into_string().unwrap();

    match std::fs::read_to_string(&filename) {
        Ok(source) => {
            let tokens = lex::lex(&source, &filename);
            println!("Tokens: {:?}", tokens);
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => error_raw!("File `{}` not found", filename.blue()),
            ErrorKind::PermissionDenied => error_raw!("No read permissions for `{}`", filename.blue()),
            _ => error_raw!("Unknown error when reading `{}`", filename.blue()),
        }
    }
}
