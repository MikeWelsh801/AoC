use std::{cmp::Ordering, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");
    let mut filter_list: Vec<&str> = contents.lines().filter(|line| !line.is_empty()).collect();

    let (div_1, div_2) = ("[[2]]", "[[6]]");
    filter_list.push(div_1);
    filter_list.push(div_2);
    filter_list.sort_by(|l1, l2| get_order(l1, l2));

    let i1 = filter_list.iter().position(|&line| line == div_1).unwrap();
    let i2 = filter_list.iter().position(|&line| line == div_2).unwrap();

    println!("Answer 1: {}", get_inorder_count(&contents));
    println!("Answer 2: {}", (i1 + 1) * (i2 + 1));
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
        return lists_in_order(l1, l2);
    } else if l1.starts_with("[") {
        let list2 = to_list(l2);
        return lists_in_order(l1, &list2[..]);
    } else if l2.starts_with("[") {
        let list1 = to_list(l1);
        return lists_in_order(&list1[..], l2);
    } else {
        return compare_numbers(l1, l2);
    }
}

fn compare_numbers(l1: &str, l2: &str) -> Option<bool> {
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

fn lists_in_order(list1: &str, list2: &str) -> Option<bool> {
    let items1: Vec<&str> = parse_list(list1);
    let items2: Vec<&str> = parse_list(list2);

    for index in 0..items1.len() {
        if index >= items2.len() {
            return Some(false);
        } else if let Some(in_order) = are_in_order(items1[index], items2[index]) {
            return Some(in_order);
        }
    }
    if items1.len() < items2.len() {
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
    let mut open_count = 0;
    let mut close_count = 0;

    for (index, char) in striped_list.chars().enumerate() {
        if char == '[' {
            open_count += 1;
        }
        if char == ']' {
            close_count += 1;
        }
        if char == ',' && open_count == close_count {
            items.push(&striped_list[start_index..index]);
            start_index = index + 1;
        }
    }
    items.push(&striped_list[start_index..]);
    items
}
