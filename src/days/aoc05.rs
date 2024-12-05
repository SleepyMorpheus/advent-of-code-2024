use std::cmp::Ordering;
use crate::helper::input_parser::{load_matrix_two};
use std::collections::{HashMap};

pub fn part_a(path: String) -> i32 {
    let (rules, updates) = load_matrix_two::<i32>(path, "|", ",", "");
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules {
        let a = rule.get(0).unwrap();
        let b = rule.get(1).unwrap();
        graph.entry(*a).or_insert(Vec::new()).push(*b);
    }

    updates.iter().fold(0, |acc, update| {
        acc + check_update(update, &graph).unwrap_or(0)
    })
}

fn check_update(update: &Vec<i32>, graph: &HashMap<i32, Vec<i32>>) -> Option<i32> {
    for i in 0..update.len() {
        for j in (i + 1)..update.len() {
            if let Some(neighbors) = graph.get(&update[j]) {
                if neighbors.contains(&update[i]) {
                    return None;
                }
            }
        }
    }
    Some(update[update.len() / 2])
}

fn fix_update(update: &Vec<i32>, graph: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut update = update.clone();
    update.sort_by(|a,b| {
        if graph.get(a).unwrap().contains(b) {
            Ordering::Less
        } else if graph.get(b).unwrap().contains(a) { 
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    update[update.len() / 2]
}

pub fn part_b(path: String) -> i32 {
    let (rules, updates) = load_matrix_two::<i32>(path, "|", ",", "");
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules {
        let a = rule.get(0).unwrap();
        let b = rule.get(1).unwrap();
        graph.entry(*a).or_insert(Vec::new()).push(*b);
    }

    updates.iter().fold(0, |acc, update| {
        if check_update(update, &graph).is_some() {
            return acc;
        }
        acc + fix_update(update, &graph)
    })
}
