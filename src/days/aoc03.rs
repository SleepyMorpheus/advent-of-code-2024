use regex_lite::Regex;
use std::cmp::PartialEq;
use crate::lib::input_parser::load_as_string;

pub fn part_a(path: String) -> i32 {
    let regex = Regex::new(r"mul\([1-9][0-9]{0,2},[1-9][0-9]{0,2}\)").unwrap();
    let mut sum = 0;
    for occ in regex.find_iter(&*load_as_string(path)) {
        let mut parts = occ.as_str().replace("mul(", "").replace(")", "");
        let mut parts = parts.split(",");
        sum += parts.next().unwrap().parse::<i32>().unwrap()
            * parts.next().unwrap().parse::<i32>().unwrap()
    }
    sum
}

#[derive(PartialEq)]
enum Mode {
    DO,
    DONT,
}

pub fn part_b(path: String) -> i32 {
    let mut sum = 0;
    let mut prev_instr = Mode::DO;
    let regex_mul = Regex::new(r"(mul\([1-9][0-9]{0,2},[1-9][0-9]{0,2}\))|(do\(\))|don't\(\)").unwrap();
    for occ in regex_mul.find_iter(&*load_as_string(path)) {
        let occ = occ.as_str();
        match (occ) {
            "do()" => prev_instr = Mode::DO,
            "don't()" => prev_instr = Mode::DONT,
            _ => {
                if prev_instr == Mode::DONT {
                    continue;
                }
                let mut parts = occ.replace("mul(", "").replace(")", "");
                let mut parts = parts.split(",");
                sum += parts.next().unwrap().parse::<i32>().unwrap()
                    * parts.next().unwrap().parse::<i32>().unwrap()
            }
        }
    }
    sum
}
