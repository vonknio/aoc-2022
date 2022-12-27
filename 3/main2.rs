use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use std::env;
use std::collections::HashSet;
use std::iter::FromIterator;

fn priority(ch: u8) -> u8 {
    if ch > 96 {
        ch - 96
    } else {
        ch - 38
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines: Result<Vec<String>, _> = BufReader::new(file).lines().collect();
    let lines = lines.expect("Couldn't read lines.");

    let mut sum: u32 = 0;
    for i in (0..lines.len() - 2).step_by(3) {
        let a: HashSet<u8> = HashSet::from_iter(lines[i].as_bytes().iter().cloned());
        let b: HashSet<u8> = HashSet::from_iter(lines[i + 1].as_bytes().iter().cloned());
        let c: HashSet<u8> = HashSet::from_iter(lines[i + 2].as_bytes().iter().cloned());
        let ab: HashSet<u8> = a.intersection(&b).copied().collect();
        sum += ab.intersection(&c).fold(0, |acc, x| acc + priority(*x) as u32);
    }

    println!("{sum}");
}
