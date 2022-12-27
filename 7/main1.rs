use std::fs::File;
use std::path::Path;
use std::env;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};

const THRESHOLD: u32 = 100000;

struct Node {
    size: u32,
    parent: usize,
    children: HashMap<String, usize>,
}

impl Node {
    fn cd(&self, dest: &str) -> usize {
        if dest == "/" {
            0
        } else if dest == ".." {
            self.parent
        } else {
            *self.children.get(dest).expect("Child not found.")
        }
    }

    fn add_child(&mut self, name: &str, idx: usize) {
        self.children.insert(String::from(name), idx);
    }
}

fn dfs(tree: &Vec<Node>, cur: usize) -> (u32, u32) {
    let mut child_sum: u32 = tree[cur].size;
    let mut node_contribution: u32 = 0;
    for (_, child) in &tree[cur].children {
        let (a, b) = dfs(tree, *child);
        child_sum += a;
        node_contribution += b;
    }
    if tree[cur].children.len() > 0 && child_sum <= THRESHOLD {
        node_contribution += child_sum;
    }

    (child_sum, node_contribution)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut tree = vec![Node {
            size: 0,
            parent: 0,
            children: HashMap::new(),
        }];

    let mut cur_node = 0;

    for line_res in lines {
        if let Ok(line) = line_res {
            if line.starts_with("$ cd") {
                cur_node = tree[cur_node].cd(&line[5..line.len()]);
            } else if !line.starts_with("$") {
                if line.len() < 3 {
                    continue;
                }
                let (a, name) = line.split_once(" ").unwrap();
                let size: u32 = a.parse().unwrap_or(0);
                tree.push(Node {
                    size: size,
                    parent: cur_node,
                    children: HashMap::new()
                });
                let idx = tree.len() - 1;
                tree[cur_node].add_child(name, idx);
            }
        }
    }

    let (overall_size, sum) = dfs(&tree, 0);
    println!("{overall_size}, {sum}");
}
