use crate::helper::input_parser::{load_matrix, load_matrix_chars, load_matrix_two};
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn part_a(path: String) -> i32 {
    let (rules, updates) = load_matrix_two::<i32>(path, "|", ",", "");
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules {
        let a = rule.get(0).unwrap();
        let b = rule.get(1).unwrap();
        // insert the node b into the neigbour list of a
        graph.entry(*a).or_insert(Vec::new()).push(*b);
    }

    updates.iter().fold(0, |acc, update| {
        acc + check_update(update, &graph).unwrap_or(0)
    })
}

fn check_update(update: &Vec<i32>, graph: &HashMap<i32, Vec<i32>>) -> Option<i32> {
    for i in 0..update.len() {
        for j in (i + 1)..update.len() {
            // Check if update[j] is a direct neighbor of update[i]
            if let Some(neighbors) = graph.get(&update[j]) {
                if neighbors.contains(&update[i]) {
                    // Dependency found, return early
                    return None;
                }
            }
        }
    }
    // If no dependency is found, return the middle element of the update
    Some(update[update.len() / 2])
}

fn fix_update(update: &mut Vec<i32>, graph: &HashMap<i32, Vec<i32>>) -> bool {
    for i in 0..update.len() {
        for j in (i + 1)..update.len() {
            // Check if update[j] is a direct neighbor of update[i]
            if let Some(neighbors) = graph.get(&update[i]) {
                if neighbors.contains(&update[j]) {
                    // Swap the values at indices i and j
                    update.swap(i, j);
                    return true;
                }
            }
        }
    }
    false
}

pub fn part_b(path: String) -> i32 {
    let (rules, updates) = load_matrix_two::<i32>(path, "|", ",", "");
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules {
        let a = rule.get(0).unwrap();
        let b = rule.get(1).unwrap();
        // Insert the node b into the neighbor list of a
        graph.entry(*a).or_insert(Vec::new()).push(*b);
    }

    updates.iter().fold(0, |acc, update| {
        if check_update(update, &graph).is_some() {
            return acc;
        }

        let mut update = update.clone();
        loop {
            if !fix_update(&mut update, &graph) {
                break;
            }
        }

        acc + update[update.len() / 2]
    })
}
