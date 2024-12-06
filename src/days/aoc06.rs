use crate::helper::input_parser::{load_matrix_chars};
use std::collections::{HashSet};

const MOVEMENTS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];


fn find_starting_position(matrix: &Vec<Vec<char>>) -> (i32, i32) {
    matrix.iter().enumerate().fold((0,0), |acc, (i, row)| {
        if let Some(j) = row.iter().position(|&x| x == '^') {
            return (i as i32,j as i32);
        }
        acc
    })
}

pub fn part_a(path: String) -> i32 {
    let matrix = &mut load_matrix_chars(path);
    
    let pos_start = find_starting_position(matrix);
    let mut pos = pos_start;
    let mut dir = 0;
    
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    loop {
        visited.insert(pos);
        
        let next_pos = (pos.0 + MOVEMENTS[dir].0,pos.1 + MOVEMENTS[dir].1);
        if !(0 <= next_pos.0 && next_pos.0 < matrix.len() as i32 && 0 <= next_pos.1 && next_pos.1 < matrix.len() as i32) {
            break
        }
        
        if matrix[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            dir = (dir + 1) % 4;
        } else {
            pos = next_pos
        }
    }


    visited.len() as i32
}

fn visits_start(matrix: &Vec<Vec<char>>, start: (i32, i32)) -> bool {
    let width = matrix.len();
    let width_i32 = matrix.len() as i32;
    let mut pos = start;
    let mut dir = 0;

    let mut visited = vec![false; width * width * 4];
    
    loop {
        let idx = (pos.0 as usize) * width * 4 + (pos.1 as usize) * 4 + dir;
        if visited[idx] {
            return true
        }
        visited[idx] = true;
        
        let next_pos = (pos.0 + MOVEMENTS[dir].0,pos.1 + MOVEMENTS[dir].1);
        if !(0 <= next_pos.0 && next_pos.0 < width_i32 && 0 <= next_pos.1 && next_pos.1 < width_i32) {
            break
        }

        if matrix[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            dir = (dir + 1) % 4;
        } else {
            pos = next_pos
        }
    }
    false
}

pub fn part_b(path: String) -> i32 {
    let matrix = &mut load_matrix_chars(path);

    let pos_start = find_starting_position(matrix);
    let mut obstacles = 0;
    
    for i in 0..(matrix.len()-1) {
        for j in 0..(matrix.len()-1) {
            if matrix[i][j] == '.' {
                matrix[i][j] = '#';
                if visits_start(matrix, pos_start) {
                    obstacles += 1;
                }
                matrix[i][j] = '.';
            }
        }
    }

    obstacles
}