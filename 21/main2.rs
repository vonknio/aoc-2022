use std::fs::File;
use std::path::Path;
use std::env;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};

enum Operation {
    Plus, Minus, Multiply, Divide, Equal
}

impl Operation {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operation::Plus => a + b,
            Operation::Minus => a - b,
            Operation::Multiply => a * b,
            Operation::Divide => a / b,
            Operation::Equal => { panic!("= on dfs"); }
        }
    }

    fn reverse(&self, a: i64, b: i64, left: bool) -> i64 {
        match self {
            Operation::Plus => a - b,
            Operation::Minus => if left { a + b } else { b - a },
            Operation::Multiply => a / b,
            Operation::Divide => if left { a * b } else { b / a},
            Operation::Equal => b,
        }
    }
}

struct Node {
    val: i64,
    operation: Operation,
    children: Option<(String, String)>,
    sibling: String,
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

fn follow_to_human(tree: &Vec<Node>, names_to_idx: &HashMap<String, usize>, path_to_human: &Vec<String>, idx_on_path: usize, parent_val: i64) -> i64 {
    let cur = &path_to_human[idx_on_path];
    if cur == "humn" {
        return parent_val;
    }
    let next = &path_to_human[idx_on_path + 1];

    let (ch1, ch2) = &tree[names_to_idx[cur]].children.as_ref().unwrap();
    let sibling = if next == ch1 { ch2 } else { ch1 };
    let left = next == ch1;
    let sibling_val = dfs(tree, names_to_idx, sibling);

    follow_to_human(
        tree, names_to_idx, path_to_human,
        idx_on_path + 1,
        tree[names_to_idx[cur]].operation.reverse(parent_val, sibling_val, left)
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut tree = Vec::<Node>::new();
    let mut names_to_idx = HashMap::<String, usize>::new();
    let mut parents = HashMap::<String, String>::new();

    for line_res in lines {
        if let Ok(line) = line_res {
            let (name, rest) = line.split_once(": ").unwrap();
            if rest.contains(" ") {
                let (ch1, rest) = rest.split_once(" ").unwrap();
                let (op, ch2) = rest.split_once(" ").unwrap();
                tree.push(Node {
                    val: 0,
                    operation: if name == "root" {
                        Operation::Equal
                    } else {
                            match op {
                            "+" => Operation::Plus,
                            "-" => Operation::Minus,
                            "*" => Operation::Multiply,
                            "/" => Operation::Divide,
                            _ => { panic!("wrong operation"); }
                        }
                    },
                    children: Some((String::from(ch1), String::from(ch2))),
                    sibling: String::new(),
                });
                parents.insert(String::from(ch1), String::from(name));
                parents.insert(String::from(ch2), String::from(name));
            } else {
                tree.push(Node {
                    val: rest.parse().unwrap(),
                    operation: Operation::Plus,
                    children: None,
                    sibling: String::new(),
                });
            }
            names_to_idx.insert(String::from(name), tree.len() - 1);
        }
    }

    for i in 0..tree.len() {
        if let Some((ch1, ch2)) = tree[i].children.clone() {
            tree[names_to_idx[&ch1]].sibling = String::from(&ch2);
            tree[names_to_idx[&ch2]].sibling = String::from(&ch1);
        }
    }

    let mut path_to_human = Vec::<String>::new();
    let mut cur = String::from("humn");
    loop {
        path_to_human.push(String::from(&cur));
        if !parents.contains_key(&cur) {
            break;
        }
        let parent = &parents[&cur];
        cur = String::from(parent);
    }
    path_to_human.reverse();

    let res = follow_to_human(&tree, &names_to_idx, &path_to_human, 0, 0);
    println!("{res}");
}
