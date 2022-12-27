use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, BufRead};
use std::collections::VecDeque;

type Item = usize;

#[derive(Clone, Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

impl Operation {
    fn apply(&self, x: usize) -> usize {
        match self {
            Operation::Add(y) => x + y,
            Operation::Multiply(y) => x * y,
            Operation::Square => x * x
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    test_int: usize,
    if_yes: usize,
    if_no: usize,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: VecDeque::new(),
            operation: Operation::Add(0),
            test_int: 1,
            if_yes: 0,
            if_no: 0,
        }
    }

    // Returns (item, destination monkey)
    fn inspect_item(&mut self) -> Option<(Item, usize)> {
        if self.items.is_empty() {
            None
        } else {
            let item = self.items.pop_front().unwrap();
            let item = self.operation.apply(item) / 3;
            if item % self.test_int == 0 {
                Some((item, self.if_yes))
            } else {
                Some((item, self.if_no))
            }
        }
    }
}

fn get_int(line: &str) -> usize {
    line.split(" ").last().unwrap().parse().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey = Monkey::new();
    let mut init = false;

    for line_res in lines {
        if let Ok(line) = line_res {
            if line.starts_with("Monkey") {
                if init {
                    monkeys.push(monkey.clone());
                    monkey = Monkey::new();
                }
                init = true;
            } else if line.contains("Starting items") {
                let (_, items) = line.split_once(": ").unwrap();
                let items = items.split(", ");
                monkey.items = items.map(|x| x.parse().unwrap()).collect();
            } else if line.contains("Operation") {
                let (_, op) = line.split_once(" = ").unwrap();
                let op = if op == "old * old" {
                    Operation::Square
                } else {
                    let op_int: usize = get_int(&line);
                    if op.chars().nth(4).unwrap() == '+' {
                        Operation::Add(op_int)
                    } else {
                        Operation::Multiply(op_int)
                    }
                };
                monkey.operation = op;
            } else if line.contains("Test") {
                monkey.test_int = get_int(&line);
            } else if line.contains("If true") {
                monkey.if_yes = get_int(&line);
            } else if line.contains("If false") {
                monkey.if_no = get_int(&line);
            }
        }
    }
    monkeys.push(monkey.clone());

    let mut monkey_business: Vec<u32> = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            loop {
                match monkeys[i].inspect_item() {
                    Some((item, j)) => {
                        monkey_business[i] += 1;
                        monkeys[j].items.push_back(item);
                    }
                    None => break,
                }
            }
        }
    }

    monkey_business.sort_by(|x, y| y.cmp(x));
    println!("{}", monkey_business[0] * monkey_business[1]);
}
