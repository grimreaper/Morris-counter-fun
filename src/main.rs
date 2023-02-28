extern crate core;

use clap::{command, Arg, ArgAction};

fn cmd() -> clap::Command {
    command!().arg(
        Arg::new("word")
            .short('w')
            .long("word")
            .action(ArgAction::Set)
            .value_name("WORD"),
    )
}

fn main() {
    let args = argfile::expand_args_from(wild::args_os(), argfile::parse_fromfile, argfile::PREFIX)
        .unwrap();
    let matches = cmd().get_matches_from(args);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        cmd().debug_assert()
    }
}
