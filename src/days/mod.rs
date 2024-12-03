use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod aoc01;
mod aoc02;
mod aoc03;
mod aoc04;
mod aoc05;
mod aoc06;
mod aoc07;
mod aoc08;
mod aoc09;
mod aoc10;
mod aoc11;
mod aoc12;

pub fn run(day: i32, first: bool, test: bool) -> Result<i32, io::Error> {
    let data = load_data(day, test)?;
    let data_refs: Vec<&str> = data.iter().map(String::as_str).collect();

    let output = match (day, first) {
        (1, true) => aoc01::aoc01_a(data_refs),
        (1, false) => aoc01::aoc01_b(data_refs),
        (2, true) => aoc02::aoc02_a(data_refs),
        (2, false) => aoc02::aoc02_b(data_refs),
        (3, true) => aoc03::part_a(data_refs),
        (3, false) => aoc03::part_b(data_refs),
        (4, true) => aoc04::part_a(data_refs),
        (4, false) => aoc04::part_b(data_refs),
        (5, true) => aoc05::part_a(data_refs),
        (5, false) => aoc05::part_b(data_refs),
        (6, true) => aoc06::part_a(data_refs),
        (6, false) => aoc06::part_b(data_refs),
        (7, true) => aoc07::part_a(data_refs),
        (7, false) => aoc07::part_b(data_refs),
        (8, true) => aoc08::part_a(data_refs),
        (8, false) => aoc08::part_b(data_refs),
        (9, true) => aoc09::part_a(data_refs),
        (9, false) => aoc09::part_b(data_refs),
        (10, true) => aoc10::part_a(data_refs),
        (10, false) => aoc10::part_b(data_refs),
        (11, true) => aoc11::part_a(data_refs),
        (11, false) => aoc11::part_b(data_refs),
        (12, true) => aoc12::part_a(data_refs),
        (12, false) => aoc12::part_b(data_refs),
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
