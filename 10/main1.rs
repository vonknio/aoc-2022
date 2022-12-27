use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut counter: i32 = 0;
    let mut register: i32 = 1;
    let mut aggregate = 0;
    let checkpoints = [20, 60, 100, 140, 180, 220];
    for line_res in lines {
        if let Ok(line) = line_res {
            counter += 1;
            if checkpoints.contains(&counter) {
                aggregate += counter * register;
            }
            if line != "noop" {
                counter += 1;
                if checkpoints.contains(&counter) {
                    aggregate += counter * register;
                }
                let (_, x) = line.split_once(" ").unwrap();
                let x: i32 = x.parse().unwrap();
                register += x;
            }
        }
    }

    println!("{aggregate}");
}
