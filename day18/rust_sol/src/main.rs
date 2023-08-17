use std::fs;

fn main() {
    let contents = fs::read_to_string("test.txt").expect("Couldn't read file.");
}
