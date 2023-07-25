use std::{cmp::Ordering, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");
    let mut filtered_list: Vec<&str> = contents.lines().filter(|line| !line.is_empty()).collect();

    let (div_1, div_2) = ("[[2]]", "[[6]]");
    filtered_list.push(div_1);
    filtered_list.push(div_2);
    filtered_list.sort_by(|l1, l2| get_order(l1, l2));

    let line_num1 = filtered_list.iter().position(|&line| line == div_1).unwrap() + 1;
    let line_num2 = filtered_list.iter().position(|&line| line == div_2).unwrap() + 1;

    println!("Answer 1: {}", get_inorder_count(&contents));
    println!("Answer 2: {}", line_num1 * line_num2);
}

fn get_order(l1: &str, l2: &str) -> Ordering {
    let ord = are_in_order(l1, l2);
    match ord {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        None => Ordering::Equal,
    }
}

fn get_inorder_count(contents: &String) -> usize {
    let iter = contents.lines().filter(|line| !line.is_empty());
    let mut iter2 = contents.lines().filter(|line| !line.is_empty());
    iter2.next();

    iter.step_by(2)
        .zip(iter2.step_by(2))
        .enumerate()
        .filter(|(_, (l1, l2))| are_in_order(l1, l2).unwrap_or(true))
        .map(|(index, _)| index + 1)
        .sum()
}

fn are_in_order(l1: &str, l2: &str) -> Option<bool> {
    if l1.starts_with("[") && l2.starts_with("[") {
        return are_lists_in_order(l1, l2);
    } else if l1.starts_with("[") {
        return are_lists_in_order(l1, &to_list(l2)[..]);
    } else if l2.starts_with("[") {
        return are_lists_in_order(&to_list(l1)[..], l2);
    } else {
        return are_numbers_in_order(l1, l2);
    }
}

fn are_numbers_in_order(l1: &str, l2: &str) -> Option<bool> {
    let num1 = l1.parse::<i32>().unwrap();
    let num2 = l2.parse::<i32>().unwrap();

    if num1 == num2 {
        return None;
    }
    Some(num1 < num2)
}

fn to_list(l2: &str) -> String {
    let mut result = String::from("[");
    result.push_str(l2);
    result.push_str("]");
    result
}

fn are_lists_in_order(list1: &str, list2: &str) -> Option<bool> {
    let list1_items = parse_list(list1);
    let list2_items = parse_list(list2);

    for index in 0..list1_items.len() {
        if index >= list2_items.len() {
            return Some(false);
        } else if let Some(in_order) = are_in_order(list1_items[index], list2_items[index]) {
            return Some(in_order);
        }
    }

    if list1_items.len() < list2_items.len() {
        return Some(true);
    }
    None
}

fn parse_list(list: &str) -> Vec<&str> {
    let mut items = vec![];
    let striped_list = &list[1..list.len() - 1];

    if striped_list.is_empty() {
        return items;
    }

    let mut start_index = 0;
    let mut open_brackets = 0;

    striped_list
        .chars()
        .enumerate()
        .for_each(|(index, char)| match char {
            '[' => open_brackets += 1,

            ']' => open_brackets -= 1,

            ',' if open_brackets == 0 => {
                items.push(&striped_list[start_index..index]);
                start_index = index + 1;
            }
            _ => (),
        });
    items.push(&striped_list[start_index..]);
    items
}
