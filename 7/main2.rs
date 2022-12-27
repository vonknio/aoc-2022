use std::fs::File;
use std::path::Path;
use std::env;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};

const CAPACITY: u32 = 70000000;
const FREE_TARGET: u32 = 30000000;
// TAKEN calculated in part 1
const TAKEN: u32 = 43313415;

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
    let mut min: u32 = 0;
    for (_, child) in &tree[cur].children {
        let (a, b) = dfs(tree, *child);
        child_sum += a;
        if b > 0 && (min == 0 || b < min) {
            min = b;
        }
    }
    if min == 0 && tree[cur].children.len() > 0 && CAPACITY - TAKEN + child_sum >= FREE_TARGET {
        min = child_sum;
    }

    (child_sum, min)
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

    let (_, min) = dfs(&tree, 0);
    println!("{min}");
}
