extern crate core;

use std::{fs, io};
use std::io::Stdin;
use std::process::exit;
use clap::{command, Arg, ArgAction};

fn cmd() -> clap::Command {
    command!().arg(
        Arg::new("input")
            .short('i')
            .long("input")
            .action(ArgAction::Set)
            .value_name("INPUT")
            .required(true),
    )
}

fn main() {
    let args = argfile::expand_args_from(wild::args_os(), argfile::parse_fromfile, argfile::PREFIX)
        .unwrap();
    let matches = cmd().get_matches_from(args);

    if let Some(input_filename) = matches.get_one::<String>("input") {
        let rdr: Box<dyn io::Read> = match input_filename.as_str() {
            "-" => Box::new(io::stdin()),
            _ => Box::new(fs::File::open(input_filename).unwrap()),
        };
    } else {
        cmd().render_help();
        exit(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        cmd().debug_assert()
    }
}
