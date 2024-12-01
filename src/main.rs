use clap::Parser;
use crate::cli::Cli;
use crate::days::run;

pub mod days;
mod cli;

fn main() {
    println!("AdventOfCode 2024: Rust Edition");
    
    let args = Cli::parse();
    
    println!("Running day {} - Part {}", args.day, if args.second { 2 } else { 1 });
    let out = run(args.day, !args.second, args.test);
    
    match out {
        Ok(output) => println!("Output: {}", output),
        Err(e) => eprintln!("Error: {}", e),
    }
}
