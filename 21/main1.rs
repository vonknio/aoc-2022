use std::fs::File;
use std::path::Path;
use std::env;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};

enum Operation {
    Plus, Minus, Multiply, Divide
}

impl Operation {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operation::Plus => a + b,
            Operation::Minus => a - b,
            Operation::Multiply => a * b,
            Operation::Divide => a / b,
        }
    }
}

struct Node {
    val: i64,
    operation: Operation,
    children: Option<(String, String)>,
}

fn dfs(tree: &Vec<Node>, names_to_idx: &HashMap<String, usize>, cur: &str) -> i64 {
    let node = &tree[names_to_idx[cur]];
    match &node.children {
        Some((ch1, ch2)) => node.operation.apply(
            dfs(tree, names_to_idx, &ch1),
            dfs(tree, names_to_idx, &ch2)
        ),
        None => node.val,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut tree = Vec::<Node>::new();
    let mut names_to_idx = HashMap::<String, usize>::new();

    for line_res in lines {
        if let Ok(line) = line_res {
            let (name, rest) = line.split_once(": ").unwrap();
            if rest.contains(" ") {
                let (ch1, rest) = rest.split_once(" ").unwrap();
                let (op, ch2) = rest.split_once(" ").unwrap();
                tree.push(Node {
                    val: 0,
                    operation: match op {
                        "+" => Operation::Plus,
                        "-" => Operation::Minus,
                        "*" => Operation::Multiply,
                        "/" => Operation::Divide,
                        _ => { panic!("wrong operation"); }
                    },
                    children: Some((String::from(ch1), String::from(ch2))),
                });
            } else {
                tree.push(Node {
                    val: rest.parse().unwrap(),
                    operation: Operation::Plus,
                    children: None,
                })
            }
            names_to_idx.insert(String::from(name), tree.len() - 1);
        }
    }

    let res = dfs(&tree, &names_to_idx, "root");
    println!("{res}");
}
