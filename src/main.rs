use std::env;
use std::process;
use clap::ArgAction;
use  clap::{Arg,App};
use ssgrep::{Config,run};

fn main() {
    let matches=App::new("SSgrep Program")
        .version("0.0.1")
        .author("Sherlock Shemol <shemol106@gmail.com>")
        .about("Search for a matched context in a file.")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .action(ArgAction::Set)
            .help("A cool file?"))
        .arg(Arg::new("ignore_case")
            .short('i')
            .long("ignore_case")
            .action(ArgAction::Set)
            .help("true to ignore case, false to not"))
        .arg(Arg::new("help")
            .long("help")
            .global(true)
            .action(ArgAction::Help) )
        .getmatches();
        
    let args:Vec<String> = env::args().collect();
    // all input from user is untrustable
    
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}",config.query);
    println!("file path is {}",config.file_path);

    if let Err(e) = run(config){
        println!("Application error: {e}");
        process::exit(1);
    }
}