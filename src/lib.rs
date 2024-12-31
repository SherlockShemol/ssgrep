use clap::{ArgAction, Parser};
use std::env;
use std::error::Error;
use std::fs;

#[derive(Parser)]
#[command(
    name = "SSgrep Program",
    version = "0.0.1",
    author = "Sherlock Shemol <shemol106@gmail.com>",
    about = "Search for matched context in a file.For author to learn Rust."
)]
pub struct Cli {
    pub query: String,

    #[arg(short = 'f', long = "file", action = ArgAction::Set, help = "Specify the file to search.")]
    pub file: String,

    #[arg(short = 'i', long = "ignore_case", action = ArgAction::SetTrue, help = "Ignore case when searching.")]
    pub ignore_case: bool,
}

pub struct Config<'a> {
    pub query: &'a String,
    pub file_path: &'a String,
    pub ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(cli: &'a Cli) -> Result<Config<'a>, &str> {
        let query = &cli.query;
        let file_path = &cli.file;
        let ignore_case = cli.ignore_case || env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.file_path).expect("Should have able to read the file.");

    let results = if config.ignore_case {
        search_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }
    results
}
