use std::collections::HashMap;

pub fn aoc01_a(data: Vec<&str>) -> i32 {
    let mut list_a: Vec<i32> = vec![];
    let mut list_b: Vec<i32> = vec![];

    let _ = data.iter().for_each(|x| {
        let mut splitter = x.split("   ");
        list_a.push(splitter.next().unwrap().parse::<i32>().unwrap());
        list_b.push(splitter.next().unwrap().parse::<i32>().unwrap());
    });

    list_a.sort();
    list_b.sort();

    let tot_diff = list_a
        .iter()
        .zip(list_b.iter())
        .fold(0, |acc, (a, b)| acc + (a - b).abs());

    tot_diff
}

pub fn aoc01_b(data: Vec<&str>) -> i32 {
    let mut list_a: Vec<i32> = vec![];
    let mut map_b: HashMap<i32, i32> = HashMap::new();

    let _ = data.iter().for_each(|x| {
        let mut splitter = x.split("   ");
        list_a.push(splitter.next().unwrap().parse::<i32>().unwrap());

        let item_b = splitter.next().unwrap().parse::<i32>().unwrap();
        map_b.insert(item_b, map_b.get(&item_b).unwrap_or(&0) + 1);
    });

    let mut tot_diff = 0;
    list_a.iter().for_each(|x| match map_b.get(x) {
        Some(v) => tot_diff += x * v,
        None => (),
    });

    tot_diff
}
