use clap::Parser;
use ssgrep::{run, Cli, Config};
use std::process;

fn main() {
    let cli = Cli::parse();

    let config = Config::build(&cli).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    });
}
