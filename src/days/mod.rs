use std::fs;
use std::io;

mod aoc01;
mod aoc02;

pub fn run(day: i32, first: bool, test: bool) -> Result<i32, io::Error> {
    //let data = load_data(day, test)?;
    //let data = data.iter().map(|x| x.as_str()).collect::<Vec<&str>>();

    let output = match (day, first) {
        //(1, true) => aoc01::aoc01_a(data),
        //(1, false) => aoc01::aoc01_b(data),
        //(2, true) => aoc02::aoc02_a(data),
        //(2, false) => aoc02::aoc02_b(data),
        _ => -1,
    };

    Ok(output)
}

fn load_data(day: i32, test: bool) -> Result<Vec<String>, io::Error> {
    let path = match test {
        true => format!("data/aoc{:02}_test.txt", day),
        false => format!("data/aoc{:02}.txt", day),
    };

    let content = fs::read_to_string(path)?; // Propagate errors with `?`
    Ok(content.lines().map(|line| line.to_owned()).collect()) // U
}
