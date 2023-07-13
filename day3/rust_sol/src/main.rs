use std::fs;

fn main() {
    // part 1

    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");
    let mut sum = 0;

    contents.lines().for_each(|line| {
        let mid = line.len() / 2;
        let comp1 = &line[..mid];
        let comp2 = &line[mid..];

        sum += get_priority(get_common_char(comp1, comp2, None).unwrap());
    });

    println!("Answer1: {sum}");

    // part 2

    let mut group: Vec<&str> = Vec::with_capacity(3);
    let mut sum2 = 0;

    contents.lines().for_each(|line| {
        group.push(line);

        if group.len() == 3 {
            let badge = get_common_char(group[0], group[1], Some(group[2])).unwrap();
            sum2 += get_priority(badge);
            group.clear();
        }
    });

    println!("Answer2: {sum2}");
}

fn get_priority(letter: char) -> u32 {
    let num = letter as u32;

    if letter.is_lowercase() {
        num - 96
    } else {
        num - 38
    }
}

fn get_common_char(s1: &str, s2: &str, s3: Option<&str>) -> Option<char> {
    for b1 in s1.bytes() {
        match s3 {
            Some(s3) => {
                if s2.contains(b1 as char) && s3.contains(b1 as char) {
                    return Some(b1 as char);
                }
            }
            None => {
                if s2.contains(b1 as char) {
                    return Some(b1 as char);
                }
            }
        }
    }
    None
}
