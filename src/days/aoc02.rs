use std::cmp::PartialEq;
use crate::helper::input_parser::load_matrix;

#[derive(PartialOrd, PartialEq)]
enum Change {
    UNDETERMINED,
    INCREASE,
    DECREASE,
}

pub fn aoc02_a(path: String) -> i32 {
    load_matrix(path, " ").iter().fold(0, |acc, x| {
        match check(x, None) { 
            true => acc + 1,
            false => acc,
        }
    })
}

pub fn aoc02_b(path: String) -> i32 {
    load_matrix(path, " ").iter().fold(0, |mut acc, x| {
        match check(x, None) {
            true => acc + 1,
            false => {
                for i in 0..(x.len() as i32) {
                    if check(x, Some(i)) {
                        acc += 1;
                        break;
                    }
                }
                acc
            }
        }
    })
}

fn check(arr: &[i32], except: Option<i32>) -> bool {
    let except_index = except.filter(|&x| x >= 0 && (x as usize) < arr.len());

    let mut iter = arr.iter().enumerate().filter_map(|(idx, &val)| {
        if Some(idx) == except_index.map(|x| x as usize) {
            None // Skip the `except` index
        } else {
            Some(val)
        }
    });

    let mut prev = match iter.next() {
        Some(first) => first,
        None => return true, // Empty or single element array is trivially valid
    };

    let mut last_change = Change::UNDETERMINED;

    for current in iter {
        let diff = current - prev;
        if last_change != Change::DECREASE && 0 < diff && diff < 4 {
            last_change = Change::INCREASE;
        } else if last_change != Change::INCREASE && -4 < diff && diff < 0 {
            last_change = Change::DECREASE;
        } else {
            return false;
        }
        prev = current;
    }
    true
}