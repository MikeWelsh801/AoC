use std::{
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

    let (p1_flow_total, p2_flow_total) = run_sim(graph, timeout);

    println!("Answer 1: {p1_flow_total}");
    println!("Answer 2: {p2_flow_total}");
}

fn run_sim(graph: HashMap<String, Valve>, timeout: i32) -> (i32, i32) {
    let root = "AA";
    let nodes = graph
        .iter()
        .filter(|(_, val)| val.flow_rate != 0)
        .map(|(key, _)| key.clone())
        .collect();

    let mut cache: HashMap<(String, Vec<String>, i32), i32> = HashMap::new();
    let part1 = get_max_flow(&graph, root, timeout, nodes, &mut cache);

    let part2 = get_permutations(&graph)
        .into_iter()
        .map(|(s1, s2)| {
            if s1.is_empty() || s2.is_empty() {
                return 0;
            }
            let elf_flow = get_max_flow(&graph, root, 26, s1, &mut cache);
            if elf_flow < part1 / 2 {
                return 0;
            }
            let eleph_flow = get_max_flow(&graph, root, 26, s2, &mut cache);
            elf_flow + eleph_flow
        })
        .max()
        .unwrap();

    (part1, part2)
}

fn get_permutations(graph: &HashMap<String, Valve>) -> Vec<(Vec<String>, Vec<String>)> {
    let nodes = graph
        .iter()
        .filter(|(_, val)| val.flow_rate != 0)
        .map(|(key, _)| key.clone())
        .collect();
    get_subsets(nodes)
}

fn get_subsets(nodes: Vec<String>) -> Vec<(Vec<String>, Vec<String>)> {
    let mut result: Vec<Vec<String>> = vec![vec![nodes[0].clone()]];

    for node in &nodes[1..] {
        let mut new_result = result.clone();
        for r in result {
            let mut cloned = r.clone();
            cloned.push(node.clone());
            new_result.push(cloned);
        }
        result = new_result;
    }

    result
        .into_iter()
        .map(|left| {
            let nodes_cloned = nodes.clone();
            let right = nodes_cloned
                .into_iter()
                .filter(|x| !left.contains(x))
                .collect();

            (left, right)
        })
        .collect()
}

fn get_max_flow(
    graph: &HashMap<String, Valve>,
    start: &str,
    time_remaining: i32,
    valves_to_visit: Vec<String>,
    cache: &mut HashMap<(String, Vec<String>, i32), i32>,
) -> i32 {
    let mut max_flow = 0;

    let key = (String::from(start), valves_to_visit.clone(), time_remaining);
    if let Some(val) = cache.get(&key) {
        return *val;
    }

    valves_to_visit.iter().enumerate().for_each(|(i, valve)| {
        let dist = (&graph[start].distances)[valve] + 1;
        let curr_time_remaining = time_remaining - dist;

        if curr_time_remaining > 0 {
            let mut curr_valves_to_visit = valves_to_visit.clone();
            curr_valves_to_visit.remove(i);

            let curr_flow = get_max_flow(
                &graph,
                valve,
                curr_time_remaining,
                curr_valves_to_visit,
                cache,
            );

            let curr_max_flow = curr_flow + get_edge_weight(&graph[valve], curr_time_remaining);
            max_flow = max_flow.max(curr_max_flow);
        }
    });

    cache.insert(key, max_flow);
    max_flow
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
        .filter(|(key, _)| graph[key].flow_rate != 0)
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
    graph.iter_mut().for_each(|(id, valve)| {
        let dist = bfs(&g2, &id);
        valve.set_distances(dist);
    });

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
