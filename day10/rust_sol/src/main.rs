use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");
    let mut cycle = 0;
    let mut reg_x = 1;
    let mut signal_stren_total = 0;

    println!("Answer 2: \n");

    contents.lines().for_each(|line| {
        let (instruction, value) = parse_line(line);
        match instruction {
            "noop" => {
                cycle += 1;
                draw_crt(cycle, reg_x);
                update_noop(cycle, reg_x, &mut signal_stren_total);
            }
            "addx" => {
                cycle += 2;
                draw_crt(cycle - 1, reg_x);
                draw_crt(cycle, reg_x);
                update_add(cycle, reg_x, &mut signal_stren_total);
                reg_x += value;
            }
            _ => (),
        }
    });
    println!("");
    println!("Answer 1: {signal_stren_total}");
}

fn update_noop(cycle: i32, reg_x: i32, signal_stren_total: &mut i32) {
    if (cycle - 20) % 40 == 0 {
        *signal_stren_total += cycle * reg_x;
    }
}

fn update_add(cycle: i32, reg_x: i32, signal_stren_total: &mut i32) {
    let prev_cycle = cycle - 1;

    if (cycle - 20) % 40 == 0 {
        *signal_stren_total += cycle * reg_x;
    } else if (prev_cycle - 20) % 40 == 0 {
        *signal_stren_total += prev_cycle * reg_x;
    }
}

fn draw_crt(cycle: i32, reg_x: i32) {
    let col = (cycle - 1) % 40;
    let (left, mid, right) = (reg_x - 1, reg_x, reg_x + 1);

    if col == left || col == mid || col == right {
        print!("#");
    } else {
        print!(".");
    }

    if col == 39 {
        println!("");
    }
}

fn parse_line(line: &str) -> (&str, i32) {
    if line.contains("noop") {
        let instruction = line.trim();
        return (instruction, 0);
    }

    let mut split = line.split_whitespace();
    let instruction = split.next().unwrap();
    let value = split.next().unwrap().parse::<i32>().unwrap();
    (instruction, value)
}
