use crate::helper::input_parser::load_as_string;

fn part_a_helper(data: String) -> i32 {
    data.split("mul(")
        .skip(1)
        .map(|x| parse_mul(x))
        .sum()
}

pub fn part_a(path: String) -> i32 {
    part_a_helper(load_as_string(path))
}


pub fn part_b(path: String) -> i32 {
    load_as_string(path).split("do()").map(|x| {
        part_a_helper(x.split_once("don't()")
            .map_or(x.to_string(),|(y, _)| y.to_string()))
    }).sum::<i32>()
}

// Accepts the part that follows mul( and returns the product of the two numbers
pub fn parse_mul(txt: &str) -> i32 {
    let mut chars = txt.chars();
    let mut num_a = String::new();
    let mut num_b = String::new();

    while let Some(digit) = chars.next() {
        if !digit.is_digit(10) {
            if digit != ',' {
                return 0;
            }
            break;
        }
        num_a.push(digit);
    }

    while let Some(digit) = chars.next() {
        if !digit.is_digit(10) {
            if digit != ')' {
                return 0;
            }
            break;
        }
        num_b.push(digit);
    }

    num_a.parse::<i32>().unwrap() * num_b.parse::<i32>().unwrap()
}