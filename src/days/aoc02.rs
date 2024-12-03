use std::cmp::PartialEq;

#[derive(PartialOrd, PartialEq)]
enum Change {
    UNDETERMINED,
    INCREASE,
    DECREASE,
}

pub fn aoc02_a(data: Vec<&str>) -> i32 {
    let mut n_save = 0;

    let _ = data.iter().for_each(|x| {
        let splitter = x.split(" ");
        let arr: Vec<i32> = splitter
            .map(|y| y.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if check(arr.clone(), None) {
            n_save += 1;
        }
    });
    n_save
}

pub fn aoc02_b(data: Vec<&str>) -> i32 {
    let mut n_save = 0;

    let _ = data.iter().for_each(|x| {
        let splitter = x.split(" ");
        let arr: Vec<i32> = splitter
            .map(|y| y.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if check(arr.clone(), None) {
            n_save += 1;
        } else {
            for i in 0..(arr.len() as i32) {
                if check(arr.clone(), Some(i)) {
                    n_save += 1;
                    break;
                }
            }
        }
    });

    n_save
}

fn check(mut arr: Vec<i32>, except: Option<i32>) -> bool {
    if let Some(x) = except {
        if !(x < 0 || arr.len() <= x as usize) {
            arr.remove(x as usize);
        }
    }
    let mut last_change = Change::UNDETERMINED;
    for i in 1..arr.len() {
        let diff = *arr.get(i).unwrap() - *arr.get(i - 1).unwrap();
        if last_change != Change::DECREASE && 0 < diff && diff < 4 {
            last_change = Change::INCREASE;
        } else if last_change != Change::INCREASE && -4 < diff && diff < 0 {
            last_change = Change::DECREASE
        } else {
            return false;
        }
    }
    true
}
