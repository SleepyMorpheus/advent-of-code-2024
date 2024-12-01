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
