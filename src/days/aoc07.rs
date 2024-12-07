use crate::helper::input_parser::{load_as_vec, load_matrix_chars, load_matrix_two};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::collections::HashSet;

fn parse_line(line: &str) -> (i64, Vec<i64>) {
    let (left, right) = line.split_once(": ").unwrap();
    let right = right.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
    (left.parse::<i64>().unwrap(), right)
}

fn recursive(num: &[i64], curr: i64, cmp: i64) -> bool {
    match num.split_first() {
        None => curr == cmp,
        Some((x, xs)) => {
            recursive(xs, curr * x, cmp) || recursive(xs, curr + x, cmp)
        }
    }
}

pub fn part_a(path: String) -> i32 {
    let result = load_as_vec(path).iter().fold(0, |acc, line| {
        let (left, right) = parse_line(line);
        match recursive(&*right, 0, left) { 
            true => acc + left,
            false => acc
        }
    });
    println!("{}", result);
    -2
}

pub fn part_b(path: String) -> i32 {
    0
}
