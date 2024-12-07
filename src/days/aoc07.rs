use crate::helper::input_parser::load_as_vec;

fn parse_line(line: &str) -> (i64, Vec<i64>) {
    let (left, right) = line.split_once(": ").unwrap();
    let right = right
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    (left.parse::<i64>().unwrap(), right)
}

enum Op {
    M,
    A,
}

fn remove_suffix_if_matches(left: u64, right: u64) -> Option<u64> {
    // ignoring case if right == 0 :)

    let digits = right.ilog10() + 1;
    let modulus = 10u64.pow(digits);

    if left % modulus == right {
        Some(left / modulus)
    } else {
        None
    }
}

fn recursive(num: &[i64], curr: i64, last_op: Op, allow_concat: bool) -> bool {
    match num.split_first() {
        None => match last_op {
            Op::A => curr == 0,
            Op::M => curr == 1,
        },
        Some((x, xs)) => {
            if let Some(suffix) = remove_suffix_if_matches(curr as u64, *x as u64) {
                if allow_concat && recursive(xs, suffix as i64, last_op, allow_concat) {
                    return true;
                }
            }

            if curr % x == 0 {
                if recursive(xs, curr / x, Op::M, allow_concat) {
                    return true;
                }
            }
            if curr - x >= 0 {
                if recursive(xs, curr - x, Op::A, allow_concat) {
                    return true;
                }
            }
            false
        }
    }
}

pub fn part_a(path: String) -> i64 {
    load_as_vec(path).iter().fold(0, |acc, line| {
        let (left, mut right) = parse_line(line);
        right.reverse();
        let res = recursive(&*right, left, Op::M, false);
        match res {
            true => acc + left,
            false => acc,
        }
    })
}

pub fn part_b(path: String) -> i64 {
    load_as_vec(path).iter().fold(0, |acc, line| {
        let (left, mut right) = parse_line(line);
        right.reverse();
        let res = recursive(&*right, left, Op::M, true);
        match res {
            true => acc + left,
            false => acc,
        }
    })
}
