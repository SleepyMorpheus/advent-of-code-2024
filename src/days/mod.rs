use crate::helper::input_parser::resolve_path;
use std::io::{self};

mod aoc01;
mod aoc02;
mod aoc03;
mod aoc04;
mod aoc05;
mod aoc06;
mod aoc07;

pub fn run(day: i32, first: bool, test: bool) -> Result<i64, io::Error> {
    let path = resolve_path(day, test);

    let output = match (day, first) {
        (1, true) => aoc01::aoc01_a(path) as i64,
        (1, false) => aoc01::aoc01_b(path) as i64,
        (2, true) => aoc02::aoc02_a(path) as i64,
        (2, false) => aoc02::aoc02_b(path) as i64,
        (3, true) => aoc03::part_a(path) as i64,
        (3, false) => aoc03::part_b(path) as i64,
        (4, true) => aoc04::part_a(path) as i64,
        (4, false) => aoc04::part_b(path) as i64,
        (5, true) => aoc05::part_a(path) as i64,
        (5, false) => aoc05::part_b(path) as i64,
        (6, true) => aoc06::part_a(path) as i64,
        (6, false) => aoc06::part_b(path) as i64,
        (7, true) => aoc07::part_a(path),
        (7, false) => aoc07::part_b(path),
        _ => -1,
    };

    Ok(output)
}
