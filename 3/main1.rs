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
    let lines = BufReader::new(file).lines();

    for line_res in lines {
        if let Ok(line) = line_res {
            let line: Vec<u8> = line.as_bytes().iter().map(|x| priority(*x)).collect();
            let a: HashSet<u8> = HashSet::from_iter(line[.. line.len() / 2].iter().cloned());
            let b: HashSet<u8> = HashSet::from_iter(line[line.len() / 2 ..].iter().cloned());
            sum += a.intersection(&b).fold(0, |acc, x| acc + *x as u32);
        }
    }

    println!("{sum}");
}
