use std::fs;

#[derive(Copy, Clone)]
enum TetrisType {
    HLine,
    Plus,
    Angle,
    VLine,
    Box,
}

#[derive(Clone, PartialEq, Eq)]
enum Element {
    Air,
    Rock,
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

struct Cave {
    map: Vec<Vec<Element>>,
    highest_unit: usize,
    current_push: usize,
    push_list: String,
}

impl Cave {
    fn new(max_x: usize, max_y: usize, push_list: String) -> Self {
        let map = vec![vec![Element::Air; max_x]; max_y];
        Cave {
            map,
            highest_unit: 0,
            current_push: 0,
            push_list: push_list.trim().to_string(),
        }
    }

    fn add_tetris_block(&mut self, position: Point, shape: TetrisType) {
        let mut block = TetrisPeice::new(shape, position);
        let mut block_falling = true;

        while block_falling {
            self.push_block(&mut block);
            block_falling = self.apply_gravity(&mut block);
        }

        block.coordinates.iter().for_each(|point| {
            if point.y + 1 > self.highest_unit {
                self.highest_unit = point.y + 1;
            }

            self.map[point.y][point.x] = Element::Rock;
        });
    }

    fn push_block(&mut self, block: &mut TetrisPeice) {
        let direction = self.push_list.chars().nth(self.current_push).unwrap_or('x');

        match direction {
            '<' => self.move_piece_left(block),
            '>' => self.move_piece_right(block),
            _ => println!("Unexpected direction: {direction}"),
        };
        self.current_push = (self.current_push + 1) % self.push_list.len();
    }

    fn move_piece_left(&self, block: &mut TetrisPeice) {
        let mut new_coordinates = vec![];

        for point in block.coordinates.iter() {
            let new_x = point.x.checked_sub(1);
            if new_x == None || self.map[point.y][new_x.unwrap()] == Element::Rock {
                return;
            }
            new_coordinates.push(Point::new(new_x.unwrap(), point.y));
        }

        block.coordinates = new_coordinates;
    }

    fn move_piece_right(&self, block: &mut TetrisPeice) {
        let mut new_coordinates = vec![];

        for point in block.coordinates.iter() {
            let new_x = point.x + 1;
            if new_x >= self.map[point.y].len() || self.map[point.y][new_x] == Element::Rock {
                return;
            }
            new_coordinates.push(Point::new(new_x, point.y));
        }

        block.coordinates = new_coordinates;
    }

    fn apply_gravity(&self, block: &mut TetrisPeice) -> bool {
        let mut new_coordinates = vec![];

        for point in block.coordinates.iter() {
            let new_y = point.y.checked_sub(1);

            if new_y == None || self.map[new_y.unwrap()][point.x] == Element::Rock {
                return false;
            }
            new_coordinates.push(Point::new(point.x, new_y.unwrap()));
        }

        block.coordinates = new_coordinates;
        true
    }

    fn print_grid(&self) {
        (0..=self.highest_unit + 3).rev().for_each(|y| {
            print!("|");
            (0..self.map[0].len()).for_each(|x| match self.map[y][x] {
                Element::Air => print!("."),
                Element::Rock => print!("#"),
            });
            println!("|");
        });

        print!("+");
        (0..self.map[0].len()).for_each(|_| {
            print!("-");
        });
        println!("+");
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");

    let mut cave = Cave::new(7, 3600, contents);
    let mut current = TetrisType::HLine;
    (0..2022).for_each(|_| {
        cave.add_tetris_block(Point::new(2, cave.highest_unit + 3), current);
        current = get_next_type(current);
    });
    cave.print_grid();
    println!("Cave height: {}", cave.highest_unit);
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
