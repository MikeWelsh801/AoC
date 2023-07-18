use std::{collections::HashMap, fs};

struct Directory {
    id: usize,
    name: String,
    size: u32,
    children: Vec<usize>,
    parent: usize,
}

impl Directory {
    fn add_root(id: usize, name: String, tree: &mut HashMap<usize, Directory>) {
        Directory::add_child(id, name, 0, tree)
    }

    fn add_child(id: usize, name: String, parent: usize, tree: &mut HashMap<usize, Directory>) {
        let node = Directory {
            id,
            name,
            size: 0,
            children: Vec::new(),
            parent,
        };

        tree.insert(node.id, node);
    }

    fn add_file(&mut self, size: u32) {
        self.size += size;
    }

    fn get_parent(&self) -> Option<usize> {
        if self.parent == 0 {
            return None;
        }
        Some(self.parent)
    }

    fn get_child(&self, name: &str, tree: &HashMap<usize, Directory>) -> Option<usize> {
        self.children
            .iter()
            .find(|id| {
                let child = tree.get(id);
                child.is_some() && child.unwrap().name == name
            })
            .copied()
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");

    let tree = parse_tree(contents);

    println!("Answer 1: {}", total_size_of_dirs_under_max(&tree, 100_000));
    println!(
        "Answer 2: {}",
        find_smallest_dir_to_delete(tree, 70_000_000, 30_000_000)
    );
}

// returns size of smallest directory that will free up enough space
fn find_smallest_dir_to_delete(
    tree: HashMap<usize, Directory>,
    total_space: u32,
    needed_space: u32,
) -> u32 {
    let total_used = tree.get(&1).unwrap().size;
    let unused_space = total_space - total_used;
    let size_to_delete = needed_space - unused_space;

    tree.values()
        .filter(|dir| dir.size >= size_to_delete)
        .min_by(|dir1, dir2| dir1.size.cmp(&dir2.size))
        .unwrap()
        .size
}

fn total_size_of_dirs_under_max(tree: &HashMap<usize, Directory>, max_size: u32) -> u32 {
    tree.values()
        .filter(|dir| dir.size <= max_size)
        .map(|dir| dir.size)
        .sum()
}

fn parse_tree(contents: String) -> HashMap<usize, Directory> {
    let mut tree = HashMap::new();
    let (mut current_id, mut parent, mut current_dir) = (1, None, None);

    contents.lines().for_each(|line| {
        if line.starts_with("$") {
            parse_command(
                &line[2..],
                &mut parent,
                &mut current_dir,
                &mut current_id,
                &mut tree,
            );
        } else {
            parse_line(line, &current_dir, &mut tree);
        }
    });
    tree
}

fn parse_line(line: &str, current_dir: &Option<usize>, tree: &mut HashMap<usize, Directory>) {
    if let Ok(size) = line.split_whitespace().next().unwrap().parse::<u32>() {
        let mut id = current_dir.unwrap().clone();

        while let Some(node) = tree.get_mut(&id) {
            node.add_file(size);

            id = node.get_parent().unwrap_or(0);
        }
    }
}

fn parse_command(
    line: &str,
    parent: &mut Option<usize>,
    current_dir: &mut Option<usize>,
    current_id: &mut usize,
    tree: &mut HashMap<usize, Directory>,
) {
    if *current_dir == None {
        *current_dir = Some(*current_id);
        Directory::add_root(*current_id, String::from("/"), tree);
        *current_id += 1;
    } else if line.starts_with("cd") {
        change_dir(&line[3..], parent, current_dir, current_id, tree);
        *current_id += 1;
    }
}

fn change_dir(
    name: &str,
    parent: &mut Option<usize>,
    current_dir: &mut Option<usize>,
    current_id: &mut usize,
    tree: &mut HashMap<usize, Directory>,
) {
    if name == ".." {
        *current_dir = *parent;
        *parent = tree.get(&parent.unwrap()).unwrap().get_parent();
        return;
    }

    let child = tree
        .get(&current_dir.unwrap())
        .unwrap()
        .get_child(name, tree);

    *parent = *current_dir;
    match child {
        Some(node) => {
            *current_dir = Some(node);
        }
        None => {
            *current_dir = Some(*current_id);

            Directory::add_child(
                current_id.clone(),
                String::from(name),
                parent.unwrap_or(0),
                tree,
            );
        }
    }
}
