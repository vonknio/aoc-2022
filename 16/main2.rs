use std::fs::File;
use std::path::Path;
use std::env;
use std::collections::{HashMap};
use std::io::{BufReader, BufRead};
use std::cmp::max;

struct Node {
    rate: i32,
    adj: Vec<u16>,
}

type Graph = Vec<Node>;
type NameMap = HashMap<String, u16>;
// My position, elephant's position, steps left, open valves
type StateMap = HashMap<(u16, u16, i32, u16), i32>;

/**
Explore the graph with memoization and a ton of hacks.
Args:
    1. me: my index in the list of nodes
    2. elephant: elephant's index in the list of nodes
    3. me_parent: my parent in the graph traversal as an index in the list of nodes
    4. elephant_parent: elephant's parent in the graph traversal as an index in the list of nodes
    5. steps: number of minutes remaining
    6. cur_flow: the sum of the rates of all valves that are currently open
    7. graph: reference to the list of all nodes in the graph
    8. states: memoization of visited states, maps from a state to the opmimal solution from that state.
            A state is a tuple (my position, elephant's position, remaining time, bitmap of open valves)
    9. open_valves: a bitmap of open valves. for node x (open_valves & (1 << x)) is 1 iff x's valve is open.
                    I'm heavily using the fact that there are at most 15 valves that are worth opening.
                    The input graph is sorted so that the nodes with non-zero rates have the lowest indices.
                    For nodes with higher indices the bit shift will overflow to 0, which is not an issue.

Returns:
    The maximum achieved flow from the current state.
*/
fn dynamic(me: u16, elephant: u16, me_parent: u16, elephant_parent: u16, steps: i32, cur_flow: i32, graph: &Graph, states: &mut StateMap, open_valves: u16) -> i32 {
    if steps <= 0 { return 0; }
    if steps == 1 { return cur_flow; }
    // All 15 non-zero valves have been opened
    if open_valves == (1 << 15) - 1 { return cur_flow * steps; }

    // Since the elephant and I act the same, assume that 'my' index is always lower.
    // Swap to avoid state repetitions.
    let (me, elephant) = if elephant > me {
        (elephant, me)
    } else {
        (me, elephant)
    };

    let state = (me, elephant, steps, open_valves);
    if let Some(opt) = states.get(&state) {
        return *opt;
    }

    let me_node = &graph[me as usize];
    let elephant_node = &graph[elephant as usize];
    let mut opt: i32 = 0;

    for me_n in me_node.adj.iter().copied() {
        for elephant_n in elephant_node.adj.iter().copied() {
            // Both move, don't open anything
            // It doesn't make sense to go back to parent without opening anything
            if me_n != me_parent && elephant_n != elephant_parent {
                opt = max(opt, dynamic(
                        me_n, elephant_n,
                        me, elephant,
                        steps - 1, cur_flow,
                        graph, states,
                        open_valves
                ) + cur_flow);
            }

            // Open mine, don't open elephant's
            if open_valves & (1 << me) == 0 && me_node.rate > 0 && elephant_parent != elephant_n {
                opt = max(opt, dynamic(
                    me, elephant_n,
                    me, elephant,
                    steps - 1, cur_flow + me_node.rate,
                    graph, states,
                    open_valves | (1 << me)
                ) + cur_flow);
            }

            // Open elephant's, don't open mine
            if me != elephant && open_valves & (1 << elephant) == 0 && elephant_node.rate > 0 && me_parent != me_n {
                opt = max(opt, dynamic(
                    me_n, elephant,
                    me, elephant,
                    steps - 1, cur_flow + elephant_node.rate,
                    graph, states,
                    open_valves | (1 << elephant)
                ) + cur_flow);
            }

            // Open both
            if me != elephant && open_valves & (1 << me) == 0 && me_node.rate > 0 && open_valves & (1 << elephant) == 0 && elephant_node.rate > 0 {
                opt = max(opt, dynamic(
                    me_n, elephant_n,
                    me, elephant,
                    steps - 2, cur_flow + me_node.rate + elephant_node.rate,
                    graph, states,
                    open_valves | (1 << me) | (1 << elephant)
                ) + 2 * cur_flow + me_node.rate + elephant_node.rate);
            }
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

    let res = dynamic(aa_idx, aa_idx, aa_idx, aa_idx, 26, 0, &graph, &mut StateMap::new(), 0);

    println!("{res}");
}
