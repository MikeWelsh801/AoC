use std::{fs, vec};

struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    test: u128,
    true_throw: usize,
    false_throw: usize,
    inspected_items: u128,
}

struct Operation {
    operator: String,
    operand: Option<u128>,
}

impl Monkey {
    fn operate(&self, input: u128) -> u128 {
        match (&self.operation.operator[..], self.operation.operand) {
            (_, None) => input.clone() * input.clone(),
            ("*", Some(n)) => input * n,
            ("+", Some(n)) => input + n,
            (_, _) => panic!("unsuported operation."),
        }
    }

    /// Returns item value (modded to keep small) and index of recieving monkey.
    fn inspect_item(&mut self, worried: bool, modulo: u128) -> (u128, usize) {
        let mut item = self.items.remove(0);
        item = self.operate(item);
        self.inspected_items += 1;

        if !worried {
            item /= 3;
        }

        if item % self.test == 0 {
            (item % modulo, self.true_throw)
        } else {
            (item % modulo, self.false_throw)
        }
    }

    fn recieve_item(&mut self, item: u128) {
        self.items.push(item);
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");

    let mut monkeys = build_monkeys(&contents);
    let mut angry_monkeys = build_monkeys(&contents);
    let modulo = get_modulo(&mut angry_monkeys);

    (0..20).for_each(|_| play_round(&mut monkeys, false, modulo));
    (0..10000).for_each(|_| play_round(&mut angry_monkeys, true, modulo));

    monkeys.sort_by(|m1, m2| m2.inspected_items.cmp(&m1.inspected_items));
    angry_monkeys.sort_by(|m1, m2| m2.inspected_items.cmp(&m1.inspected_items));

    println!(
        "Answer 1: {}",
        monkeys[0].inspected_items * monkeys[1].inspected_items
    );
    println!(
        "Answer 2: {}",
        angry_monkeys[0].inspected_items * angry_monkeys[1].inspected_items
    );
}

fn get_modulo(agro_monkeys: &mut [Monkey]) -> u128 {
    agro_monkeys.iter().map(|m| m.test).product()
}

fn play_round(monkeys: &mut [Monkey], worried: bool, test: u128) {
    (0..monkeys.len()).for_each(|i| {
        (0..monkeys[i].items.len()).for_each(|_| {
            let (item, recvr) = monkeys[i].inspect_item(worried, test);
            monkeys[recvr].recieve_item(item);
        });
    });
}

fn build_monkeys(contents: &String) -> Vec<Monkey> {
    let mut lines = vec![];
    let mut monkeys = vec![];

    contents.lines().for_each(|line| match line.is_empty() {
        true => {
            monkeys.push(build_monkey(&lines));
            lines.clear();
        }
        false => lines.push(line),
    });
    monkeys.push(build_monkey(&lines));
    monkeys
}

fn build_monkey(lines: &[&str]) -> Monkey {
    let mut items = vec![];
    let mut operation = Operation {
        operator: String::new(),
        operand: None,
    };
    let (mut test, mut true_throw, mut false_throw) = (0, 0, 0);

    lines.iter().for_each(|line| {
        let input = line.trim();

        if input.starts_with("Starting") {
            items = parse_items(input);
        } else if input.starts_with("Operation") {
            operation = parse_operation(input);
        } else if input.starts_with("Test") {
            test = parse_number(input);
        } else if input.starts_with("If true") {
            true_throw = parse_number(input) as usize;
        } else if input.starts_with("If false") {
            false_throw = parse_number(input) as usize;
        }
    });
    Monkey {
        items,
        operation,
        test,
        true_throw,
        false_throw,
        inspected_items: 0,
    }
}

fn parse_number(input: &str) -> u128 {
    input
        .split_whitespace()
        .rev()
        .next()
        .unwrap()
        .parse::<u128>()
        .unwrap()
}

fn parse_operation(input: &str) -> Operation {
    let operation: Vec<&str> = input.split_whitespace().rev().take(2).collect();

    match operation[0].parse::<u128>() {
        Ok(num) => Operation {
            operator: String::from(operation[1]),
            operand: Some(num),
        },
        Err(_) => Operation {
            operator: String::from(operation[1]),
            operand: None,
        },
    }
}

fn parse_items(line: &str) -> Vec<u128> {
    line.split([':', ','].as_ref())
        .map(|word| word.trim().parse::<u128>())
        .filter(|item| item.is_ok())
        .map(|num| num.unwrap())
        .collect()
}
