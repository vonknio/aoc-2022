use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use std::env;

#[derive(Copy, Clone)]
struct Interval(u32, u32);

// Checks if a is fully contained in b
fn fully_contained(a: Interval, b: Interval) -> bool {
	a.0 >= b.0 && a.1 <= b.1
}

fn any_fully_contained(a: Interval, b: Interval) -> bool {
	fully_contained(a, b) || fully_contained(b, a)
}

fn parse_interval(s: &str) -> Interval {
	let (s, e) = s.split_once("-").expect("Invalid input.");
	Interval(s.parse().unwrap(), e.parse().unwrap())
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let path = Path::new(args.get(1).expect("No filename provided."));
	let file = File::open(&path).expect("Couldn't open file.");
	let lines = BufReader::new(file).lines();

	let mut sum = 0;
	for line_res in lines {
		if let Ok(line) = line_res {
			if line.len() > 6 {
				let (a, b) = line.split_once(",").expect("Invalid input");
				sum += any_fully_contained(parse_interval(a), parse_interval(b)) as u32;
			}
		}
	}
	println!("{sum}");
}