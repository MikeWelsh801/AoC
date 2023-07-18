use std::{fs, vec};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");
    let grid = create_grid(contents);
    let mut count = 0;
    let mut high_score = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if is_visible(row, col, &grid) {
                count += 1;
            }
            if is_on_edge(row, col, &grid) {
                continue;
            }
            let new_score = get_score(row, col, &grid);
            if new_score > high_score {
                high_score = new_score;
            }
        }
    }
    println!("Answer 1: {count}");
    println!("Answer 2: {high_score}");
}

fn get_score(row: usize, col: usize, grid: &[Vec<u32>]) -> u32 {
    get_left_score(row, col, grid) *
    get_right_score(row, col, grid) *
    get_up_score(row, col, grid) *
    get_down_score(row, col, grid)
}

fn get_left_score(row: usize, col: usize, grid: &[Vec<u32>]) -> u32 {
    let tree = grid[row][col];
    let mut score = 0;

    for i in (0..col).rev() {
        score += 1;
        if grid[row][i] >= tree {
            break;
        }
    }
    score
}

fn get_right_score(row: usize, col: usize, grid: &[Vec<u32>]) -> u32 {
    let tree = grid[row][col];
    let mut score = 0;

    for i in col+1..grid[row].len() {
        score += 1;
        if grid[row][i] >= tree {
            break;
        }
    }
    score
}

fn get_up_score(row: usize, col: usize, grid: &[Vec<u32>]) -> u32 {
    let tree = grid[row][col];
    let mut score = 0;

    for i in (0..row).rev() {
        score += 1;
        if grid[i][col] >= tree {
            break;
        }
    }
    score
}

fn get_down_score(row: usize, col: usize, grid: &[Vec<u32>]) -> u32 {
    let tree = grid[row][col];
    let mut score = 0;

    for i in row+1..grid.len() {
        score += 1;
        if grid[i][col] >= tree {
            break;
        }
    }
    score
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
