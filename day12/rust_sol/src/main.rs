use std::{fs, vec};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");

    let (grid, s_pos, e_pos) = build_grid(&contents);
    let (path_len, prev) = bfs(&grid, s_pos, e_pos);
    let (shortest_len, shortest_prev) = get_shortest_a_len(&grid, e_pos);

    print_path(&grid, &prev, e_pos);
    println!("Answer 1: {path_len}");

    print_path(&grid, &shortest_prev, e_pos);
    println!("Answer 2: {shortest_len}");
}

fn print_path(
    grid: &Vec<Vec<char>>,
    prev: &Vec<Vec<Option<(usize, usize)>>>,
    e_pos: (usize, usize),
) {
    let mut path = vec![];
    let mut pos = e_pos;
    path.push(e_pos);

    while let Some(node) = prev[pos.0][pos.1] {
        path.push(node);
        pos = node;
    }

    (0..grid.len()).into_iter().for_each(|i| {
        (0..grid[i].len()).into_iter().for_each(|j| {
            if path.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        });
        println!();
    });

}

fn get_shortest_a_len(grid: &Vec<Vec<char>>, e_pos: (usize, usize)) -> (u32, Vec<Vec<Option<(usize, usize)>>>) {
    grid.iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, ele)| **ele == 'a')
                .map(|(j, _)| bfs(&grid, (i, j), e_pos))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn bfs(
    grid: &Vec<Vec<char>>,
    s_pos: (usize, usize),
    e_pos: (usize, usize),
) -> (u32, Vec<Vec<Option<(usize, usize)>>>) {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue = vec![];
    let mut dist = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
    let mut prev: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; grid[0].len()]; grid.len()];
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
                prev[node.0][node.1] = Some(curr);
                queue.push(*node);

                // stop when we reach the end.
                if *node == e_pos {
                    done = true;
                    return;
                }
            }
        });
    }
    (dist[e_pos.0][e_pos.1], prev)
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
