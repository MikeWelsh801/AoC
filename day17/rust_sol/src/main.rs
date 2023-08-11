use std::{collections::HashMap, fs};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum TetrisType {
    HLine,
    Plus,
    Angle,
    VLine,
    Box,
}

enum Direction {
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    fn get_bits(&self) -> u64 {
        let bits = 1 << self.x;
        let offset = self.y % 8;

        bits << offset * 8
    }
}

struct TetrisPeice {
    coordinates: Vec<Point>,
}

impl TetrisPeice {
    fn new(shape: TetrisType, position: Point) -> Self {
        let coordinates = match shape {
            TetrisType::HLine => {
                vec![
                    position,
                    Point::new(position.x + 1, position.y),
                    Point::new(position.x + 2, position.y),
                    Point::new(position.x + 3, position.y),
                ]
            }
            TetrisType::Plus => {
                vec![
                    Point::new(position.x, position.y + 1),
                    Point::new(position.x + 1, position.y + 1),
                    Point::new(position.x + 2, position.y + 1),
                    Point::new(position.x + 1, position.y),
                    Point::new(position.x + 1, position.y + 2),
                ]
            }
            TetrisType::Angle => {
                vec![
                    position,
                    Point::new(position.x + 1, position.y),
                    Point::new(position.x + 2, position.y),
                    Point::new(position.x + 2, position.y + 1),
                    Point::new(position.x + 2, position.y + 2),
                ]
            }
            TetrisType::VLine => {
                vec![
                    position,
                    Point::new(position.x, position.y + 1),
                    Point::new(position.x, position.y + 2),
                    Point::new(position.x, position.y + 3),
                ]
            }
            TetrisType::Box => {
                vec![
                    position,
                    Point::new(position.x, position.y + 1),
                    Point::new(position.x + 1, position.y),
                    Point::new(position.x + 1, position.y + 1),
                ]
            }
        };

        TetrisPeice { coordinates }
    }
}

struct Cave<'a> {
    map: Vec<u64>,
    highest_unit: usize,
    current_push: usize,
    push_list: &'a str,
}

impl<'a> Cave<'a> {
    fn new(max_y: usize, push_list: &'a String) -> Self {
        let map = vec![0; max_y / 8 + 1];
        Cave {
            map,
            highest_unit: 0,
            current_push: 0,
            push_list: push_list.trim(),
        }
    }

    fn add_tetris_block(&mut self, position: Point, shape: TetrisType) {
        let mut block = TetrisPeice::new(shape, position);
        let mut block_falling = true;

        while block_falling {
            let direction = &self.push_list[self.current_push..=self.current_push];
            match direction {
                "<" => self.move_piece(&mut block, Direction::Left),
                ">" => self.move_piece(&mut block, Direction::Right),
                _ => panic!("Unexpected direction: {direction}"),
            };
            self.current_push = (self.current_push + 1) % self.push_list.len();

            block_falling = self.move_piece(&mut block, Direction::Down);
        }

        block.coordinates.iter().for_each(|point| {
            if point.y + 1 > self.highest_unit {
                self.highest_unit = point.y + 1;
            }

            self.map[point.y / 8] = self.map[point.y / 8] ^ point.get_bits();
        });
    }

    fn move_piece(&self, block: &mut TetrisPeice, direction: Direction) -> bool {
        let mut new_coordinates = vec![];

        for point in block.coordinates.iter() {
            let y_index = point.y / 8;
            match direction {
                Direction::Right => {
                    let new_x = point.get_bits() << 1;

                    // if the line has a 1 where the new posiiton would be, xor would flip
                    // the 1 bit to zero, decreasing the value of the line
                    if point.x == 6 || self.map[y_index] ^ new_x < self.map[y_index] {
                        return false;
                    }
                    new_coordinates.push(Point::new(point.x + 1, point.y));
                }
                Direction::Left => {
                    let new_x = point.get_bits() >> 1;

                    if point.x == 0 || self.map[y_index] ^ new_x < self.map[y_index] {
                        return false;
                    }
                    new_coordinates.push(Point::new(point.x - 1, point.y));
                }
                Direction::Down => {
                    let new_y = point.y.checked_sub(1);

                    if new_y == None {
                        return false;
                    }

                    let new_y = new_y.unwrap();
                    let new_x = Point::new(point.x, new_y).get_bits();
                    if self.map[new_y / 8] ^ new_x < self.map[new_y / 8] {
                        return false;
                    }
                    new_coordinates.push(Point::new(point.x, new_y));
                }
            };
        }
        block.coordinates = new_coordinates;
        true
    }

    fn print_grid(&self) {
        (0..self.highest_unit + 3).rev().for_each(|y| {
            print!("|");
            let offset = (y % 8) * 8;
            let line = self.map[y / 8] >> offset;

            (0..7).for_each(|bit| match (1 << bit) & line {
                0 => print!("."),
                _ => print!("#"),
            });
            println!("|");
        });
        println!("+-------+");
    }

    fn covers_width(&self, row: usize) -> bool {
        let mut lines = 0;

        (0..8).for_each(|offset| {
            let line = self.map[row] >> (offset * 8);
            lines = lines | line;
        });

        lines & 127 == 127
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");

    // *** Part 1 ***

    let mut cave = Cave::new(0xE10, &contents);
    let mut current = TetrisType::HLine;
    (0..2022).for_each(|_| {
        cave.add_tetris_block(Point::new(2, cave.highest_unit + 3), current);
        current = get_next_type(current);
    });
    cave.print_grid();
    println!("Answer 1: {}", cave.highest_unit);

    // *** Part 2 *** find height after dropping 1 trillion pieces.
    const MAX_ITER: usize = 0xE8D4A51000; // 1 trillion

    let mut cave = Cave::new(0x186A0, &contents);
    let mut current = TetrisType::HLine;
    let mut map: HashMap<(u64, TetrisType, usize), (usize, usize)> = HashMap::new();
    let mut cycle_height = 0;

    for piece_count in 0..MAX_ITER {
        cave.add_tetris_block(Point::new(2, cave.highest_unit + 3), current);
        current = get_next_type(current);
        let row = (cave.highest_unit - 1) / 8;

        if !cave.covers_width(row) {
            continue;
        }

        // add to map to check for cycles (key's already in map)
        let key = (cave.map[row], current, cave.current_push);
        let val = (piece_count, cave.highest_unit);

        // found a cycle, insert returns previous values
        if let Some((prev_count, prev_height)) = map.insert(key, val) {
            let iter_per_cycle = piece_count - prev_count;
            let number_of_cycles = (MAX_ITER - piece_count) / iter_per_cycle - 1;

            cycle_height = (cave.highest_unit - prev_height) * number_of_cycles;

            // calculating the height of all the cycles we haven't encountered yet
            // allows us to skip a ton of iterations.
            let up_to = piece_count + (iter_per_cycle * number_of_cycles) + 1;
            (up_to..MAX_ITER).for_each(|_| {
                cave.add_tetris_block(Point::new(2, cave.highest_unit + 3), current);
                current = get_next_type(current);
            });
            break;
        }
    }
    println!("Answer 2: {}", cave.highest_unit + cycle_height);
}

fn get_next_type(current_type: TetrisType) -> TetrisType {
    match current_type {
        TetrisType::HLine => TetrisType::Plus,
        TetrisType::Plus => TetrisType::Angle,
        TetrisType::Angle => TetrisType::VLine,
        TetrisType::VLine => TetrisType::Box,
        TetrisType::Box => TetrisType::HLine,
    }
}
