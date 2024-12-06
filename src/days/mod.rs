use crate::helper::input_parser::resolve_path;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod aoc01;
mod aoc02;
mod aoc03;
mod aoc04;
mod aoc05;
mod aoc06;

pub fn run(day: i32, first: bool, test: bool) -> Result<i32, io::Error> {
    let path = resolve_path(day, test);

    let output = match (day, first) {
        (1, true) => aoc01::aoc01_a(path),
        (1, false) => aoc01::aoc01_b(path),
        (2, true) => aoc02::aoc02_a(path),
        (2, false) => aoc02::aoc02_b(path),
        (3, true) => aoc03::part_a(path),
        (3, false) => aoc03::part_b(path),
        (4, true) => aoc04::part_a(path),
        (4, false) => aoc04::part_b(path),
        (5, true) => aoc05::part_a(path),
        (5, false) => aoc05::part_b(path),
        (6, true) => aoc06::part_a(path),
        (6, false) => aoc06::part_b(path),
        _ => -1,
    };

    Ok(output)
}
