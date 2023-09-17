use std::fs;

const GRID_SIZE: usize = 27;

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
    grid: Vec<Vec<Vec<BlockType>>>,
    side_coordinates: Vec<(i32, i32, i32)>,
}

#[derive(Clone, PartialEq)]
enum BlockType {
    Air,
    Lava,
    Water,
}

impl Map {
    fn new() -> Self {
        let grid = vec![vec![vec![BlockType::Air; GRID_SIZE]; GRID_SIZE]; GRID_SIZE];
        Map {
            grid,
            side_coordinates: vec![],
        }
    }

    fn add_point(&mut self, &Point { x, y, z }: &Point) {
        self.grid[x][y][z] = BlockType::Lava;

        // remove any sizes that contain this point
        self.side_coordinates
            .retain(|&elem| elem != (x as i32, y as i32, z as i32));

        // remove shared sides

        // up in z direction
        if self.grid[x][y][z + 1] == BlockType::Lava {
            // remove all elements that match that lava element
            self.side_coordinates
                .retain(|&elem| elem != (x as i32, y as i32, z as i32 + 1));
        } else {
            self.side_coordinates
                .push((x as i32, y as i32, z as i32 + 1));
        }

        // down in z direction
        if z > 0 && self.grid[x][y][z - 1] == BlockType::Lava {
            // remove all elements that match that lava element
            self.side_coordinates
                .retain(|&elem| elem != (x as i32, y as i32, z as i32 - 1));
        } else {
            self.side_coordinates
                .push((x as i32, y as i32, z as i32 - 1));
        }

        // up in y direction
        if self.grid[x][y + 1][z] == BlockType::Lava {
            // remove all elements that match that lava element
            self.side_coordinates
                .retain(|&elem| elem != (x as i32, (y + 1) as i32, z as i32));
        } else {
            self.side_coordinates
                .push((x as i32, y as i32 + 1, z as i32));
        }

        // down in y direction
        if y > 0 && self.grid[x][y - 1][z] == BlockType::Lava {
            // remove all elements that match that lava element
            self.side_coordinates
                .retain(|&elem| elem != (x as i32, y as i32 - 1, z as i32));
        } else {
            self.side_coordinates
                .push((x as i32, y as i32 - 1, z as i32));
        }

        // up in x direction
        if self.grid[x + 1][y][z] == BlockType::Lava {
            // remove all elements that match that lava element
            self.side_coordinates
                .retain(|&elem| elem != (x as i32 + 1, y as i32, z as i32));
        } else {
            self.side_coordinates
                .push((x as i32 + 1, y as i32, z as i32));
        }

        // down in x direction
        if x > 0 && self.grid[x - 1][y][z] == BlockType::Lava {
            // remove all elements that match that lava element
            self.side_coordinates
                .retain(|&elem| elem != (x as i32 - 1, y as i32, z as i32));
        } else {
            self.side_coordinates
                .push((x as i32 - 1, y as i32, z as i32));
        }
    }

    fn get_side_total(&self) -> usize {
        self.side_coordinates.iter().count()
    }

    fn remove_air(&mut self) {
        (0..GRID_SIZE - 1).for_each(|idx| self.fill_water(0, 0, idx));
        (0..GRID_SIZE - 1).for_each(|idx| self.fill_water(0, idx, 0));
        (0..GRID_SIZE - 1).for_each(|idx| self.fill_water(idx, 0, 0));

        // remove all of the non water elements.
        let new_coord: Vec<(i32, i32, i32)> = self
            .side_coordinates
            .iter()
            .filter(|(x, y, z)| {
                // out of bounds or not water
                *x < 0
                    || *y < 0
                    || *z < 0
                    || self.grid[*x as usize][*y as usize][*z as usize] == BlockType::Water
            })
            .map(|coordinate| coordinate.clone())
            .collect();

        self.side_coordinates = new_coord;
    }

    fn fill_water(&mut self, x: usize, y: usize, z: usize) {
        self.grid[x][y][z] = BlockType::Water;

        // fill up y direction
        if y < GRID_SIZE - 2 && self.grid[x][y + 1][z] == BlockType::Air {
            self.fill_water(x, y + 1, z);
        }
        // fill down y direction
        if y > 0 && self.grid[x][y - 1][z] == BlockType::Air {
            self.fill_water(x, y - 1, z);
        }

        // fill up x direction
        if x < GRID_SIZE - 2 && self.grid[x + 1][y][z] == BlockType::Air {
            self.fill_water(x + 1, y, z);
        }
        // fill down x direction
        if x > 0 && self.grid[x - 1][y][z] == BlockType::Air {
            self.fill_water(x - 1, y, z);
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");
    let mut map = Map::new();

    contents
        .lines()
        .for_each(|line| map.add_point(&parse_line(line)));

    println!("Answer 1: {}", map.get_side_total());
    map.remove_air();
    println!("Answer 2: {}", map.get_side_total());
}

fn parse_line(line: &str) -> Point {
    let mut split = line.split(',');
    let x = split.next().unwrap().parse::<usize>().unwrap();
    let y = split.next().unwrap().parse::<usize>().unwrap();
    let z = split.next().unwrap().parse::<usize>().unwrap();

    Point::new(x, y, z)
}
