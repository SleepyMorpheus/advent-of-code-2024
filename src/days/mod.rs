use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod aoc01;
mod aoc02;

pub fn run(day: i32, first: bool, test: bool) -> Result<i32, io::Error> {
    let data = load_data(day, test)?;
    let data_refs: Vec<&str> = data.iter().map(String::as_str).collect();

    let output = match (day, first) {
        (1, true) => aoc01::aoc01_a(data_refs),
        (1, false) => aoc01::aoc01_b(data_refs),
        (2, true) => aoc02::aoc02_a(data_refs),
        (2, false) => aoc02::aoc02_b(data_refs),
        _ => -1,
    };

    Ok(output)
}

fn load_data(day: i32, test: bool) -> Result<Vec<String>, io::Error> {
    let path = if test {
        format!("data/aoc{:02}_test.txt", day)
    } else {
        format!("data/aoc{:02}.txt", day)
    };

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect::<Result<Vec<_>, _>>()
}