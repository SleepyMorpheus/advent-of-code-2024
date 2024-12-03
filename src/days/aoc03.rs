use std::cmp::{PartialEq};
use regex::Regex;

pub fn part_a(data: Vec<&str>) -> i32 {
    println!("{:?}", data.join("\n"));
    let regex = Regex::new(r"mul\([1-9][0-9]{0,2},[1-9][0-9]{0,2}\)").unwrap();
    let mut sum = 0;
    for occ in regex.find_iter(&*data.join("\n")) {
        println!("{:?}", occ);
        let mut parts = occ.as_str().replace("mul(", "").replace(")", "");
        let mut parts = parts.split(",");
        sum += parts.next().unwrap().parse::<i32>().unwrap() * parts.next().unwrap().parse::<i32>().unwrap()
    }
    sum
}

#[derive(PartialEq)]
enum Mode {
    DO,
    DONT
}

pub fn part_b(data: Vec<&str>) -> i32 {
    let input = data.join("\n");
    println!("{:?}", input);

    // Compile the regexes
    let regex_mul = Regex::new(r"mul\([1-9][0-9]{0,2},[1-9][0-9]{0,2}\)").unwrap();
    let regex_do = Regex::new(r"do\(\)").unwrap();
    let regex_dont = Regex::new(r"don't\(\)").unwrap();

    // Find all matches for each pattern
    let mut matches = Vec::new();

    for mat in regex_mul.find_iter(&input) {
        matches.push((mat.start(), mat.as_str()));
    }
    for mat in regex_do.find_iter(&input) {
        matches.push((mat.start(), mat.as_str()));
    }
    for mat in regex_dont.find_iter(&input) {
        matches.push((mat.start(), mat.as_str()));
    }

    // Sort matches by their start index
    matches.sort_by_key(|&(start, _)| start);
    
    
    let mut sum = 0;
    let mut prev_instr = Mode::DO;
    for (_,occ) in matches.into_iter() {
        println!("{:?}", occ);
        match occ {
            "do()" => prev_instr = Mode::DO,
            "don't()" => prev_instr = Mode::DONT,
            _ => {
                if prev_instr == Mode::DONT {
                    continue;
                }
                let mut parts = occ.replace("mul(", "").replace(")", "");
                let mut parts = parts.split(",");
                sum += parts.next().unwrap().parse::<i32>().unwrap() * parts.next().unwrap().parse::<i32>().unwrap()
            }
        }
    }
    sum
}
