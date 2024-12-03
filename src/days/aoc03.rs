use regex::Regex;
use std::cmp::PartialEq;

pub fn part_a(data: Vec<&str>) -> i32 {
    println!("{:?}", data.join("\n"));
    let regex = Regex::new(r"mul\([1-9][0-9]{0,2},[1-9][0-9]{0,2}\)").unwrap();
    let mut sum = 0;
    for occ in regex.find_iter(&*data.join("\n")) {
        println!("{:?}", occ);
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

pub fn part_b(data: Vec<&str>) -> i32 {
    let regex_mul =
        Regex::new(r"(mul\([1-9][0-9]{0,2},[1-9][0-9]{0,2}\))|(do\(\))|don't\(\)").unwrap();
    let mut sum = 0;
    let mut prev_instr = Mode::DO;
    for occ in regex_mul.find_iter(&*data.join("\n")) {
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
