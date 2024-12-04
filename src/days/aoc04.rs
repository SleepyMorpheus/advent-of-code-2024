use std::ops::Add;
use crate::helper::input_parser::{load_as_string, load_matrix};

pub fn part_a(path: String) -> i32 {
    let mut matches = 0;
    let matrix = &load_matrix::<String>(path, "");
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] != "X" {
                continue;
            }
            for dx in -1..2 {
                for dy in -1..2 {
                    if start_check(matrix, i as i32, j as i32, dx, dy) == "XMAS" {
                        matches += 1;
                    }
                }
            }
            
        }
    }
    matches
}

pub fn start_check(matrix: &Vec<Vec<String>>, mut x: i32 , mut y: i32, dx: i32, dy: i32) -> String {
    let mut output = String::with_capacity(4);
    while output.len() < 4 && 0 <= x && x < matrix.len() as i32 && 0 <= y && y < matrix[0].len() as i32 {
        output = output + matrix.get(x as usize).unwrap().get(y as usize).unwrap();
        x += dx;
        y += dy;
    }
    output
}


pub fn part_b(path: String) -> i32 {
    let mut matches = 0;
    let matrix = &load_matrix::<String>(path, "");
    for i in 1..matrix.len()-1 {
        for j in 1..matrix[0].len()-1 {
            if matrix[i][j] != "A" {
                continue;
            }

            let a = matrix[i-1][j-1] == "M" && matrix[i+1][j+1] == "S" ||  matrix[i-1][j-1] == "S" && matrix[i+1][j+1] == "M";
            let b = matrix[i+1][j-1] == "M" && matrix[i-1][j+1] == "S" ||  matrix[i+1][j-1] == "S" && matrix[i-1][j+1] == "M";
            if a && b {
                matches += 1;
            }

        }
    }
    matches
}
