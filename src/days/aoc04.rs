use crate::helper::input_parser::load_matrix_chars;

pub fn part_a(path: String) -> i32 {
    let mut matches = 0;
    let matrix = &load_matrix_chars(path);
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] != 'X' {
                continue;
            }
            for dx in -1..2 {
                for dy in -1..2 {
                    if start_check(matrix, i as i32, j as i32, dx, dy) {
                        matches += 1;
                    }
                }
            }
        }
    }
    matches
}

pub fn start_check(matrix: &Vec<Vec<char>>, mut x: i32, mut y: i32, dx: i32, dy: i32) -> bool {
    let reference = "XMAS";
    for c in reference.chars() {
        if x < 0 || y < 0 || x >= matrix.len() as i32 || y >= matrix[0].len() as i32 {
            return false;
        }
        if matrix[x as usize][y as usize] != c {
            return false;
        }
        x += dx;
        y += dy;
    }
    true
}

pub fn part_b(path: String) -> i32 {
    let mut matches = 0;
    let matrix = &load_matrix_chars(path);
    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[0].len() - 1 {
            if matrix[i][j] != 'A' {
                continue;
            }

            let a = matrix[i - 1][j - 1] == 'M' && matrix[i + 1][j + 1] == 'S'
                || matrix[i - 1][j - 1] == 'S' && matrix[i + 1][j + 1] == 'M';
            let b = matrix[i + 1][j - 1] == 'M' && matrix[i - 1][j + 1] == 'S'
                || matrix[i + 1][j - 1] == 'S' && matrix[i - 1][j + 1] == 'M';
            if a && b {
                matches += 1;
            }
        }
    }
    matches
}
