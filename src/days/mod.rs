use std::fs::File;
use std::io::{self, BufRead, BufReader};
use crate::lib::input_parser::resolve_path;

mod aoc01;
mod aoc02;
mod aoc03;

pub fn run(day: i32, first: bool, test: bool) -> Result<i32, io::Error> {
    let path = resolve_path(day, test);

    let output = match (day, first) {
        (1, true) => aoc01::aoc01_a(path),
        (1, false) => aoc01::aoc01_b(path),
        (2, true) => aoc02::aoc02_a(path),
        (2, false) => aoc02::aoc02_b(path),
        (3, true) => aoc03::part_a(path),
        (3, false) => aoc03::part_b(path),
        _ => -1,
    };

    Ok(output)
}