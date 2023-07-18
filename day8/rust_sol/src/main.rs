use std::{fs, vec};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");
    let grid = create_grid(contents);
    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if is_visible(row, col, &grid) {
                count += 1;
            }
        }
    }
    println!("Answer 1: {count}");
}

fn is_visible(row: usize, col: usize, grid: &[Vec<u32>]) -> bool {
    if is_on_edge(row, col, grid) {
        return true;
    }
    if check_left(row, col, grid) || check_right(row, col, grid) ||
       check_up(row, col, grid) || check_down(row, col, grid) {
           return true;
    }
    false
}

fn check_left(row: usize, col: usize, grid: &[Vec<u32>]) -> bool {
    let tree = grid[row][col];
    for i in 0..col {
        if grid[row][i] >= tree {
            return false;
        }
    }
    true
}

fn check_right(row: usize, col: usize, grid: &[Vec<u32>]) -> bool {
    let tree = grid[row][col];
    for i in col+1..grid[row].len() {
        if grid[row][i] >= tree {
            return false;
        }
    }
    true
}

fn check_up(row: usize, col: usize, grid: &[Vec<u32>]) -> bool {
    let tree = grid[row][col];
    for i in 0..row {
        if grid[i][col] >= tree {
            return false;
        }
    }
    true
}

fn check_down(row: usize, col: usize, grid: &[Vec<u32>]) -> bool {
    let tree = grid[row][col];
    for i in row+1..grid.len() {
        if grid[i][col] >= tree {
            return false;
        }
    }
    true
}

fn is_on_edge(row: usize, col: usize, grid: &[Vec<u32>]) -> bool {
    row == 0 || row == grid.len() - 1 || col == 0 || col == grid[row].len() - 1
}

fn create_grid(contents: String) -> Vec<Vec<u32>> {
    let mut grid = vec![];

    contents.lines().enumerate().for_each(|(index, line)| {
        grid.push(vec![]);
        line.chars().for_each(|char| {
            let num = char.to_digit(10).unwrap();
            grid[index].push(num);
        });
    });
    grid
}
