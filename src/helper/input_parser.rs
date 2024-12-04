use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufRead, Read};
use std::str::FromStr;

pub fn resolve_path(day: i32, test: bool) -> String {
    if test {
        format!("data/aoc{:02}_test.txt", day)
    } else {
        format!("data/aoc{:02}.txt", day)
    }
}

pub fn load_as_string(path: String) -> String {
    let mut file = File::open(path).unwrap();
    let mut input = String::with_capacity(30_000);
    file.read_to_string(&mut input).unwrap();
    input 
}

pub fn load_as_vec(path: String) -> Vec<String> {
    load_as_string(path).lines().map(|x| x.to_string()).collect()
}

pub fn load_matrix<T: FromStr>(path: String, sep: &str) -> Vec<Vec<T>> {
    let mut matrix = Vec::new();
    let file = File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let row = line.split(&sep).map(|x| x.parse::<T>().ok().unwrap()).collect();
        matrix.push(row);
    }
    matrix
}

pub fn load_matrix_string<>(path: String) -> Vec<Vec<char>> {
    let file = File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    reader.lines().map(|x| x.unwrap().chars().collect()).collect()
}
