use std::{cmp, fs};

struct Grid {
    map: Vec<Vec<Material>>,
    floor: usize,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            map: vec![vec![Material::Air; 650]; 1000],
            floor: 0,
        }
    }

    fn add_wall(&mut self, start: &Point, end: &Point) {
        if start.x == end.x {
            let min = cmp::min(start.y, end.y);
            let max = cmp::max(start.y, end.y);

            (min..=max).for_each(|y| self.map[start.x][y] = Material::Rock);
        } else if start.y == end.y {
            let min = cmp::min(start.x, end.x);
            let max = cmp::max(start.x, end.x);

            (min..=max).for_each(|x| self.map[x][start.y] = Material::Rock);
        }

        let new_floor = cmp::max(start.y, end.y) + 2;
        self.floor = cmp::max(new_floor, self.floor);
    }

    fn add_path(&mut self, path: Vec<Point>) {
        path.iter()
            .zip(path.iter().skip(1))
            .for_each(|(p1, p2)| self.add_wall(p1, p2));
    }

    fn add_floor(&mut self) {
        let start = Point::new(0, self.floor);
        let end = Point::new(self.map.len() - 1, self.floor);
        self.add_wall(&start, &end);
    }

    fn display_grid(&self, min: Point, max: Point) {
        (min.y..=max.y).into_iter().for_each(|y| {
            print!("{y}\t");
            (min.x..=max.x)
                .into_iter()
                .for_each(|x| match self.map[x][y] {
                    Material::Air => print!("."),
                    Material::Rock => print!("#"),
                    Material::Sand => print!("o"),
                });
            println!();
        });
    }

    fn drop_sand(&mut self, drop_point: &Point) -> bool {
        let mut sand_location = Point::new(drop_point.x, drop_point.y);

        if self.map[sand_location.x][sand_location.y] == Material::Sand {
            return false;
        }

        while sand_location.y < self.map[drop_point.x].len() - 1 {
            // try to go down
            if self.map[sand_location.x][sand_location.y + 1] == Material::Air {
                sand_location.y += 1;
            // try to go down and to the left
            } else if self.map[sand_location.x - 1][sand_location.y + 1] == Material::Air {
                sand_location.y += 1;
                sand_location.x -= 1;
            // try to go down and to the right
            } else if self.map[sand_location.x + 1][sand_location.y + 1] == Material::Air {
                sand_location.y += 1;
                sand_location.x += 1;
            } else {
                self.map[sand_location.x][sand_location.y] = Material::Sand;
                return true;
            }
        }
        false
    }

    fn count_sand_drops(&mut self) -> u32 {
        let drop_point = Point::new(500, 0);
        let mut count = 0;
        while self.drop_sand(&drop_point) {
            count += 1;
        }
        count
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Material {
    Air,
    Rock,
    Sand,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");

    let mut grid = Grid::new();
    fill_grid(contents, &mut grid);

    // part 1
    let mut count = grid.count_sand_drops();

    // uncomment out line bellow to show map
    grid.display_grid(Point::new(430, 0), Point::new(550, grid.floor));
    println!("Answer 1: {count}");

    // part 2 (just add to count from part 1, so we don't waste work)
    grid.add_floor();
    count += grid.count_sand_drops();

    // uncomment out line bellow to show map
    grid.display_grid(Point::new(400, 0), Point::new(550, grid.floor));
    println!("Answer 2: {count}");
}

fn fill_grid(contents: String, grid: &mut Grid) {
    contents
        .lines()
        .map(|line| parse_points(&line))
        .for_each(|points| grid.add_path(points));
}

fn parse_points(line: &str) -> Vec<Point> {
    let points = line.split("->");

    points
        .into_iter()
        .map(|point| {
            let mut split = point.trim().split(",");
            let x = split.next().unwrap().parse::<usize>().unwrap();
            let y = split.next().unwrap().parse::<usize>().unwrap();

            Point::new(x, y)
        })
        .collect()
}
