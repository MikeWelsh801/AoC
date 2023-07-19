use std::{collections::HashSet, fs};

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn move_up(&mut self) {
        self.y += 1;
    }

    fn move_down(&mut self) {
        self.y -= 1;
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn move_right(&mut self) {
        self.x += 1;
    }

    fn move_up_right(&mut self) {
        self.move_up();
        self.move_right();
    }

    fn move_up_left(&mut self) {
        self.move_up();
        self.move_left();
    }

    fn move_down_left(&mut self) {
        self.move_down();
        self.move_left();
    }

    fn move_down_right(&mut self) {
        self.move_down();
        self.move_right();
    }

    fn copy(&self) -> Point {
        Point::new(self.x, self.y)
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");

    let mut two_knots = vec![Point::new(0, 0); 2];
    let mut two_visited = HashSet::new();
    two_visited.insert(two_knots[0].copy());

    let mut ten_knots = vec![Point::new(0, 0); 10];
    let mut ten_visited = HashSet::new();
    ten_visited.insert(ten_knots[0].copy());

    contents.lines().for_each(|line| {
        update_position(&mut two_knots, line, &mut two_visited);
        update_position(&mut ten_knots, line, &mut ten_visited);
    });

    println!("Answer 1: {}", two_visited.len());
    println!("Answer 2: {}", ten_visited.len());
}

fn update_position(knots: &mut Vec<Point>, instructions: &str, visited: &mut HashSet<Point>) {
    let (direction, move_count) = parse_line(instructions);

    (0..move_count).for_each(|_| {
        match direction {
            "U" => knots[0].move_up(),
            "D" => knots[0].move_down(),
            "L" => knots[0].move_left(),
            "R" => knots[0].move_right(),
            _ => (),
        }
        update_knots(knots);
        visited.insert(knots.last().unwrap().copy());
    });
}

fn update_knots(knots: &mut Vec<Point>) {
    (1..knots.len()).for_each(|index| {
        let diff = (knots[index-1].x - knots[index].x, knots[index-1].y - knots[index].y);

        match diff {
            (2, 0) => knots[index].move_right(),
            (-2, 0) => knots[index].move_left(),
            (0, 2) => knots[index].move_up(),
            (0, -2) => knots[index].move_down(),

            (2, 1) | (1, 2) | (2, 2) => knots[index].move_up_right(),
            (-2, 1) | (-1, 2) | (-2, 2) => knots[index].move_up_left(),
            (2, -1) | (1, -2) | (2, -2) => knots[index].move_down_right(),
            (-2, -1) | (-1, -2) | (-2, -2) => knots[index].move_down_left(),

            (_, _) => (),
        }
    });
}

fn parse_line(instructions: &str) -> (&str, usize) {
    let mut split = instructions.split_whitespace();
    let direction = split.next().unwrap();

    let move_count = split
        .next()
        .unwrap()
        .parse::<usize>()
        .expect("Couldn't parse move count.");

    (direction, move_count)
}
