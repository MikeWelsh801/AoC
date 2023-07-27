use std::{cmp, fs};

struct Grid {
    map: Vec<(i128, i128)>,
    target_line: i128,
}

impl Grid {
    fn new(target_line: i128) -> Grid {
        Grid {
            map: vec![],
            target_line,
        }
    }

    fn add_sb_pair(&mut self, sensor: &Point, beacon: &Point) {
        let distance = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
        let dist_to_target = (sensor.y - self.target_line).abs();
        let remaining_distance = distance - dist_to_target;

        if remaining_distance < 0 {
            return;
        }

        let blockout = (sensor.x - remaining_distance, sensor.x + remaining_distance);
        self.add_block(blockout);
    }

    fn add_block(&mut self, new_block: (i128, i128)) {
        self.map.push(new_block);
        self.map.sort_by(|b1, b2| b1.0.cmp(&b2.0));
        self.merge_blocks();
    }

    fn merge_blocks(&mut self) {
        let mut items_to_remove = vec![];

        for i in 0..self.map.len() - 1 {
            if items_to_remove.contains(&i) {
                continue;
            }

            for j in i + 1..self.map.len() {
                if blocks_overlap(self.map[i], self.map[j]) {
                    let min = cmp::min(self.map[i].0, self.map[j].0);
                    let max = cmp::max(self.map[i].1, self.map[j].1);

                    self.map[i] = (min, max);
                    items_to_remove.push(j);
                }
            }
        }

        items_to_remove.sort_by(|a, b| b.cmp(&a));
        items_to_remove.iter().for_each(|index| {
            self.map.remove(*index);
        });
    }

    fn get_blocked_count(&self) -> i128 {
        self.map.iter().map(|block| block.1 - block.0).sum()
    }
}

fn blocks_overlap(b1: (i128, i128), b2: (i128, i128)) -> bool {
    b1.1 >= b2.0 && b1.1 <= b2.1 || b1.0 >= b2.0 && b1.0 <= b2.1 || b1.0 <= b2.0 && b1.1 >= b2.1
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i128,
    y: i128,
}

impl Point {
    fn new(x: i128, y: i128) -> Point {
        Point { x, y }
    }
}

struct Sensor {
    sensor: Point,
    distance: i128,
}

impl Sensor {
    fn new(sensor: Point, beacon: Point) -> Sensor {
        Sensor {
            sensor,
            distance: (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs(),
        }
    }

    fn get_perimiter(&self) -> i128 {
        self.distance + 1
    }

    fn contains_point(&self, point: &Point) -> bool {
        let dist_to_point = (self.sensor.x - point.x).abs() + (self.sensor.y - point.y).abs();
        dist_to_point <= self.distance
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");
    let grid = fill_grid(&contents, 10);
    let blocked_count = grid.get_blocked_count();
    println!("Answer 1: {blocked_count}");

    let sensors = get_sensor_list(&contents);

    let mut candidate_points = vec![];
    sensors.iter().for_each(|s1| {
        sensors.iter().for_each(|s2| {
            if s1.sensor != s2.sensor {
                let mut interect = get_intersections(&s1, &s2);
                candidate_points.append(&mut interect);
            }
        });
    });

    let filtered: Vec<&Point> = candidate_points
        .iter()
        .filter(|point| {
            point.x >= 0 && point.x <= 4_000_000 && point.y >= 0 && point.y <= 4_000_000
        })
        .filter(|point| !is_in_range(point, &sensors))
        .collect();

    let tuning_freq = filtered[0].x * 4_000_000 + filtered[0].y;
    println!("Answer 2: {tuning_freq}");
}

fn is_in_range(point: &Point, sensors: &Vec<Sensor>) -> bool {
    for sensor in sensors {
        if sensor.contains_point(point) {
            return true;
        }
    }
    false
}

fn get_intersections(s1: &Sensor, s2: &Sensor) -> Vec<Point> {
    let mut intersect = get_points(s1, s2);
    let mut other = get_points(s2, s1);
    intersect.append(&mut other);
    intersect
}

fn get_points(s1: &Sensor, s2: &Sensor) -> Vec<Point> {
    [
        (
            (s1.sensor.x - s1.get_perimiter()),
            (s2.sensor.x - s2.get_perimiter()),
        ),
        (
            (s1.sensor.x - s1.get_perimiter()),
            (s2.sensor.x + s2.get_perimiter()),
        ),
        (
            (s1.sensor.x + s1.get_perimiter()),
            (s2.sensor.x - s2.get_perimiter()),
        ),
        (
            (s1.sensor.x + s1.get_perimiter()),
            (s2.sensor.x + s2.get_perimiter()),
        ),
    ]
    .into_iter()
    .map(|(dx1, dx2)| {
        let x = (dx2 + s2.sensor.y + dx1 - s1.sensor.y) / 2;
        let y = (dx2 + s2.sensor.y - dx1 + s1.sensor.y) / 2;
        Point::new(x, y)
    })
    .collect()
}

fn get_sensor_list(contents: &str) -> Vec<Sensor> {
    let mut sensors = vec![];

    contents
        .lines()
        .map(|line| parse_points(&line))
        .for_each(|(sensor, beacon)| sensors.push(Sensor::new(sensor, beacon)));

    sensors
}

fn fill_grid(contents: &String, target_line: i128) -> Grid {
    let mut grid = Grid::new(target_line);
    contents
        .lines()
        .map(|line| parse_points(&line))
        .for_each(|(sensor, beacon)| {
            grid.add_sb_pair(&sensor, &beacon);
        });

    grid
}

fn parse_points(line: &str) -> (Point, Point) {
    let mut split = line.split(['=', ',', ':'].as_ref());

    split.next();
    let sensor_x = split.next().unwrap().parse::<i128>().unwrap();
    split.next();
    let sensor_y = split.next().unwrap().parse::<i128>().unwrap();

    split.next();
    let beacon_x = split.next().unwrap().parse::<i128>().unwrap();
    split.next();
    let beacon_y = split.next().unwrap().parse::<i128>().unwrap();

    let sensor = Point::new(sensor_x, sensor_y);
    let beacon = Point::new(beacon_x, beacon_y);
    (sensor, beacon)
}
