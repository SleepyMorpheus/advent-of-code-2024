use crate::days::run;
use std::env;

pub mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        panic!("Not enough arguments: cargo run <day: 0-24> <part: 0-1> <test: 0-1> <output: 0-1>");
    }

    let day: u8 = args[1].parse().unwrap();
    let part: u8 = args[2].parse().unwrap();
    let test: u8 = args[3].parse().unwrap();
    let debug: u8 = args[4].parse().unwrap();

    if debug == 1 {
        println!("Running day {} - Part {} - Test {}", day, part, test);
    }

    let out = run(day as i32, part == 0, test == 1);

    if debug == 1 {
        match out {
            Ok(output) => println!("Output: {}", output),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
