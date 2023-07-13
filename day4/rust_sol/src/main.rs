use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");

    println!("Answer 1: {}", get_count(&contents, is_contained));
    println!("Answer 2: {}", get_count(&contents, is_overlapping));
}

fn get_count(contents: &String, f: fn(i32, i32, i32, i32) -> bool) -> usize {
    contents
        .lines()
        .filter(|line| {
            let (first_start, first_end, second_start, second_end) = split_line(line);
            f(first_start, first_end, second_start, second_end)
        })
        .count()
}

fn is_contained(first_start: i32, first_end: i32, second_start: i32, second_end: i32) -> bool {
    (first_start <= second_start && first_end >= second_end)
        || (second_start <= first_start && second_end >= first_end)
}

fn is_overlapping(first_start: i32, first_end: i32, second_start: i32, second_end: i32) -> bool {
    is_contained(first_start, first_end, second_start, second_end)
        || (first_end >= second_start && first_end <= second_end)
        || (first_start >= second_start && first_start <= second_end)
}

fn split_line(line: &str) -> (i32, i32, i32, i32) {
    let mut split = line.split(",");
    let (first, second) = (split.next().unwrap(), split.next().unwrap());

    let (first_start, first_end) = extract_numbers(first);
    let (second_start, second_end) = extract_numbers(second);

    (first_start, first_end, second_start, second_end)
}

fn extract_numbers(text: &str) -> (i32, i32) {
    let mut split = text.split("-");

    (
        split.next().unwrap().parse::<i32>().unwrap(),
        split.next().unwrap().parse::<i32>().unwrap(),
    )
}
