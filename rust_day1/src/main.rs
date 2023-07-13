use std::fs;

fn main() {
    let contents = fs::read_to_string("elfCals.txt").expect("Couldn't read file.");

    let mut total_calories = 0;
    let mut list: Vec<i32> = vec![];
    for line in contents.split('\n') {
        if let Ok(value) = line.trim().parse::<i32>() {
            total_calories += value;
        } else {
            list.push(total_calories);
            total_calories = 0;
        }
    }

    list.sort_by(|a, b| b.cmp(a));
    println!("Answer 1: {}", list[0]);
    println!("Answer 2: {}", list[0] + list[1] + list[3]);
}
