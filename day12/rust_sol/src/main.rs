use std::{fs, vec};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");

    let (grid, s_pos, e_pos) = build_grid(&contents);
    let path_len = bfs(&grid, s_pos, e_pos);
    let shortest_len = get_shortest_a_len(&grid, e_pos);

    println!("Answer 1: {path_len}");
    println!("Answer 2: {shortest_len}");
}

fn get_shortest_a_len(grid: &Vec<Vec<char>>, e_pos: (usize, usize)) -> u32 {
    let mut min = u32::MAX;
    grid.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, ele)| {
            if *ele == 'a' {
                let shortest = bfs(&grid, (i, j), e_pos);
                if shortest < min {
                    min = shortest;
                }
            }
        })
    });
    min
}

fn bfs(grid: &Vec<Vec<char>>, s_pos: (usize, usize), e_pos: (usize, usize)) -> u32 {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = vec![];
    let mut dist = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
    let mut done = false;

    // distance to start is zero, and start has been visited.
    dist[s_pos.0][s_pos.1] = 0;
    visited[s_pos.0][s_pos.1] = true;
    queue.push(s_pos);

    while !queue.is_empty() && !done {
        let curr = queue.remove(0);

        let adj = get_adj_nodes(curr, &grid);

        adj.iter().for_each(|node| {
            if !visited[node.0][node.1] {
                visited[node.0][node.1] = true;
                dist[node.0][node.1] = dist[curr.0][curr.1] + 1;
                queue.push(*node);

                // stop when we reach the end.
                if grid[node.0][node.1] == 'E' {
                    done = true;
                    return;
                }
            }
        });
    }
    dist[e_pos.0][e_pos.1] as u32
}

fn get_adj_nodes(curr: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut adj = vec![];

    let node = grid[curr.0][curr.1] as i32;

    // up: curr.0 - 1, curr.1
    if curr.0 > 0 {
        let up = grid[curr.0 - 1][curr.1] as i32;
        if up - node <= 1 {
            adj.push((curr.0 - 1, curr.1));
        }
    }
    // down: curr.0 + 1, curr.1
    if curr.0 < grid.len() - 1 {
        let down = grid[curr.0 + 1][curr.1] as i32;
        if down - node <= 1 {
            adj.push((curr.0 + 1, curr.1));
        }
    }
    // left: curr.0, curr.1 + 1
    if curr.1 < grid[0].len() - 1 {
        let left = grid[curr.0][curr.1 + 1] as i32;
        if left - node <= 1 {
            adj.push((curr.0, curr.1 + 1));
        }
    };
    // right: curr.0, curr.1 - 1
    if curr.1 > 0 {
        let right = grid[curr.0][curr.1 - 1] as i32;
        if right - node <= 1 {
            adj.push((curr.0, curr.1 - 1));
        }
    };

    adj
}

fn build_grid(contents: &String) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let mut grid = vec![];
    let mut s_pos = (0, 0);
    let mut e_pos = (0, 0);

    contents.lines().enumerate().for_each(|(i, line)| {
        grid.push(vec![]);
        line.chars().enumerate().for_each(|(j, char)| {
            let mut height = char;

            match char {
                'S' => {
                    height = 'a';
                    s_pos = (i, j);
                }
                'E' => {
                    height = 'z';
                    e_pos = (i, j);
                }
                _ => (),
            }
            grid[i].push(height);
        })
    });
    (grid, s_pos, e_pos)
}
