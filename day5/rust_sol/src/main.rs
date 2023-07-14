use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");

    // part 1
    print!("Answer 1: ");
    print_answer(&contents, swap_stacks);

    // part 2
    print!("Answer 2: ");
    print_answer(&contents, swap_stacks_9001);
}

fn print_answer(contents: &String, swap_func: fn(&mut Vec<Vec<char>>, &str)) {
    let mut stacks: Vec<Vec<char>> = vec![
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];
    let mut filling_stacks = true;

    for line in contents.lines() {
        if line.is_empty() {
            stacks.iter_mut().for_each(|stack| {
                stack.reverse();
            });
            filling_stacks = false;
            continue;
        }

        match filling_stacks {
            true => fill_stacks(&mut stacks, line),
            false => swap_func(&mut stacks, line),
        }
    }

    (0..9).for_each(|i| {
        print!("{}", stacks[i].pop().unwrap());
    });
    println!("");
}

fn swap_stacks_9001(stacks: &mut Vec<Vec<char>>, line: &str) {
    let (move_num, from_index, to_index) = extract_nums(line);
    let mut values = Vec::new();

    (0..move_num).for_each(|_| {
        let value = stacks[from_index].pop().unwrap();
        values.push(value);
    });

    values.into_iter().rev().for_each(|val| {
        stacks[to_index].push(val);
    });
}

fn swap_stacks(stacks: &mut Vec<Vec<char>>, line: &str) {
    let (move_num, from_index, to_index) = extract_nums(line);

    (0..move_num).for_each(|_| {
        let val = stacks[from_index].pop().unwrap();
        stacks[to_index].push(val);
    });
}

fn extract_nums(line: &str) -> (usize, usize, usize) {
    let mut split = line.split_whitespace();

    split.next();
    let move_num = split.next().unwrap().parse::<usize>().unwrap();

    split.next();
    let from_index = split.next().unwrap().parse::<usize>().unwrap() - 1;

    split.next();
    let to_index = split.next().unwrap().parse::<usize>().unwrap() - 1;

    (move_num, from_index, to_index)
}

fn fill_stacks(stacks: &mut Vec<Vec<char>>, line: &str) {
    line.chars()
        .enumerate()
        .filter(|(_, char)| char.is_ascii_alphabetic())
        .for_each(|(index, char)| {
            let i = (index - 1) / 4;

            stacks[i].push(char);
        });
}
