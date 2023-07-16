use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");

    println!("Answer 1: {}", get_index(&contents, 4));
    println!("Answer 1: {}", get_index(&contents, 14));
}

fn get_index(text: &str, len: usize) -> usize {
    let mut start = 0;

    (len..text.len())
        .find(|index| {
            start += 1;
            unique_chars(&text[start - 1..*index])
        })
        .unwrap()
}

fn unique_chars(text: &str) -> bool {
    for char in text.chars() {
        let mod_text = text.rsplit(char);
        if mod_text.count() > 2 {
            return false;
        }
    }
    true
}
