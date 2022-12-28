use std::fs::File;
use std::path::Path;
use std::env;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::cmp::max;

#[derive(Debug)]
struct Node {
    rate: i32,
    adj: Vec<u16>,
}

type Graph = Vec<Node>;
type NameMap = HashMap<String, u16>;
// Current position, steps left, open valves
type StateMap = HashMap<(u16, i32, u16), i32>;

// cur_flow is per minute
fn dynamic(cur: u16, steps: i32, cur_flow: i32, graph: &Graph, states: &mut StateMap, open_valves: u16) -> i32 {
    if steps <= 0 { return 0; }
    if steps == 1 { return cur_flow; }
    // All 15 non-zero valves have been opened
    if open_valves == (1 << 15) - 1 { return cur_flow * steps; }

    let state = (cur, steps, open_valves);
    if let Some(opt) = states.get(&state) {
        return *opt;
    }
    let node = &graph[cur as usize];
    let mut opt: i32 = 0;
    for n in &node.adj {
        // Don't open current, go to n
        opt = max(opt, dynamic(*n, steps - 1, cur_flow, graph, states, open_valves) + cur_flow);
        // Open current, go to n
        if node.rate > 0 && open_valves & (1 << cur) == 0 {
            opt = max(opt, dynamic(*n, steps - 2, cur_flow + node.rate, graph, states, open_valves | (1 << cur)) + 2 * cur_flow + node.rate);
        }
    }
    states.insert(state, opt);
    return opt;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut graph_unsorted: Vec<(String, i32, Vec<String>)> = Vec::new();

    for line_res in lines {
        if let Ok(line) = line_res {
            let (node, adj) = line.split_once(";").unwrap();
            let (name, rate) = node.split_once(" has flow rate=").unwrap();
            let (_, name) = name.split_once(" ").unwrap();
            let rate: i32 = rate.parse().unwrap();
            // Remember to replace "to valve " with "to valves " in the input
            let (_, adj) = adj.split_once("valves ").unwrap();
            let adj: Vec<String> = adj.split(", ").map(String::from).collect();
            graph_unsorted.push((String::from(name), rate, adj));
        }
    }

    // Sort by decreasing valve rates
    graph_unsorted.sort_by(|a, b| b.1.cmp(&a.1));

    let mut name_map: NameMap = NameMap::new();
    let mut aa_idx: u16 = 0;

    for i in 0..graph_unsorted.len() {
        name_map.insert(String::from(&graph_unsorted[i].0), i as u16);
        if &graph_unsorted[i].0 == "AA" {
            aa_idx = i as u16;
        }
    }

    let mut graph: Graph = Graph::new();
    for (_, rate, adj) in graph_unsorted {
        graph.push(Node {
            rate: rate,
            adj: adj.iter().map(|x| name_map[x]).collect(),
        });
    }

    let res = dynamic(aa_idx, 30, 0, &graph, &mut StateMap::new(), 0);

    println!("{res}");
}
