use std::{
    cmp,
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone)]
struct Valve {
    id: String,
    flow_rate: i32,
    adj_list: Vec<String>,
    distances: HashMap<String, i32>,
}

impl Valve {
    fn new(id: String, flow_rate: i32, adj_list: Vec<String>) -> Self {
        Valve {
            id,
            flow_rate,
            adj_list,
            distances: HashMap::new(),
        }
    }

    fn set_distances(&mut self, distances: HashMap<String, i32>) {
        self.distances = distances;
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read file.");
    let graph = build_graph(&contents);
    let timeout = 30;

    let get_flow_total = run_sim(graph, timeout);
    println!("{get_flow_total}");
}

fn run_sim(graph: HashMap<String, Valve>, timeout: i32) -> i32 {
    let root = "AA";
    let mut time_remaining = timeout;

    let opened = &mut vec![];
    let paths = get_paths(&graph, root, &mut time_remaining, 0, opened);
    paths
}

fn get_paths(
    graph: &HashMap<String, Valve>,
    start: &str,
    time_remaining: &mut i32,
    current_max: i32,
    visited: &mut Vec<String>,
) -> i32 {
    let mut flow = current_max;
    let distances = &graph[start].distances;

    for (valve, dist) in distances.iter() {
        let mut curr_time = *time_remaining - (*dist + 1);
        if curr_time <= 0 {
            curr_time = 0;
        }

        if visited.contains(valve) || *time_remaining - (*dist + 1) <= 0 {
            continue;
        }

        let curr = current_max + get_edge_weight(&graph[valve], curr_time);
        let mut curr_visit = visited.clone();
        curr_visit.push(valve.clone());

        flow = cmp::max(
            flow,
            get_paths(graph, valve, &mut curr_time, curr, &mut curr_visit),
        );
    }
    flow
}

fn get_edge_weight(to_valve: &Valve, time_left: i32) -> i32 {
    time_left * to_valve.flow_rate
}

fn bfs(graph: &HashMap<String, Valve>, src: &str) -> HashMap<String, i32> {
    let mut visited = HashSet::new();
    let mut queue = vec![src];
    let mut distances = HashMap::new();

    graph.keys().for_each(|key| {
        distances.insert(key.clone(), i32::MAX);
    });

    distances.insert(src.to_string(), 0);
    visited.insert(src.to_string());

    while !queue.is_empty() {
        let node = queue.remove(0);

        graph[node].adj_list.iter().for_each(|adj_node| {
            if !visited.contains(adj_node) {
                visited.insert(adj_node.clone());
                distances.insert(adj_node.clone(), distances[node] + 1);
                queue.push(&adj_node);
            }
        });
    }

    distances
        .into_iter()
        .filter(|(key, _)| graph[&key.clone()].flow_rate != 0)
        .collect()
}

fn build_graph(contents: &str) -> HashMap<String, Valve> {
    let mut graph = HashMap::new();
    contents
        .lines()
        .map(|line| parse_line(line))
        .for_each(|valve| {
            let id = valve.id.clone();
            graph.insert(id, valve);
        });

    // store the distance from N to every other node with non-zero flow
    // in N for easy lookup
    let g2 = graph.clone();
    for (id, valve) in graph.iter_mut() {
        let dist = bfs(&g2, &id);
        valve.set_distances(dist);
    }

    graph
}

fn parse_line(line: &str) -> Valve {
    let mut split = line.split([' ', ';', '='].as_ref());

    split.next();
    let id = split.next().unwrap();

    while split.next().unwrap() != "rate" {
        continue;
    }

    let flow_rate = split.next().unwrap().parse::<i32>().unwrap();

    let mut adj_list = vec![];
    for elem in split.into_iter().rev() {
        if elem == "valves" || elem == "valve" {
            break;
        }

        let adj_valve = elem.strip_suffix(",").unwrap_or(elem);
        adj_list.push(String::from(adj_valve));
    }

    Valve::new(String::from(id), flow_rate, adj_list)
}
