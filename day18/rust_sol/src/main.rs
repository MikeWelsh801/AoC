use std::fs;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Point { x, y, z }
    }
}

struct Map {
    grid: Vec<Vec<Vec<usize>>>,
    side_total: u64,
}

impl Map {
    fn new() -> Self {
        let grid = vec![vec![vec![0; 100]; 100]; 100];
        Map {
            grid,
            side_total: 0,
        }
    }

    fn add_point(&mut self, &Point { x, y, z }: &Point) {
        self.grid[x][y][z] = 1;
        self.side_total += 6;

        // remove shared sides

        // up/down in z direction
        if self.grid[x][y][z + 1] == 1 {
            self.side_total -= 2;
        }
        if z > 0 && self.grid[x][y][z - 1] == 1 {
            self.side_total -= 2;
        }

        // up/down in y direction
        if self.grid[x][y + 1][z] == 1 {
            self.side_total -= 2;
        }
        if y > 0 && self.grid[x][y - 1][z] == 1 {
            self.side_total -= 2;
        }

        // up/down in x direction
        if self.grid[x + 1][y][z] == 1 {
            self.side_total -= 2;
        }
        if x > 0 && self.grid[x - 1][y][z] == 1 {
            self.side_total -= 2;
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");
    let mut map = Map::new();

    contents
        .lines()
        .for_each(|line| map.add_point(&parse_line(line)));
    println!("Answer 1: {}", map.side_total);

    let mut map = Map::new();

    contents
        .lines()
        .for_each(|line| map.add_point(&parse_line(line)));

    println!("Answer 2: {}", map.side_total);
}

fn parse_line(line: &str) -> Point {
    let mut split = line.split(',');
    let x = split.next().unwrap().parse::<usize>().unwrap();
    let y = split.next().unwrap().parse::<usize>().unwrap();
    let z = split.next().unwrap().parse::<usize>().unwrap();

    Point::new(x, y, z)
}
