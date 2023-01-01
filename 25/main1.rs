use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, BufRead};
use std::cmp::max;

// Little-endian
type RawRepresentation = Vec<i32>;

fn parse(s: &str) -> RawRepresentation {
    let mut raw = RawRepresentation::new();
    for c in s.chars().rev() {
        let val = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => { panic!("Wrong input"); }
        };
        raw.push(val);
    }
    raw
}

// Add two vales position-wise
fn add_raw(a: &RawRepresentation, b: &RawRepresentation) -> RawRepresentation {
    let mut res = RawRepresentation::new();
    for i in 0..max(a.len(), b.len()) {
        res.push(a.get(i).unwrap_or(&0) + b.get(i).unwrap_or(&0));
    }
    res
}

// Make each position only hold values -2 <= x <= 2
fn normalise(a: &RawRepresentation) -> RawRepresentation {
    let mut b = a.clone();
    loop {
        let mut change = false;
        for i in 0..b.len() {
            let val = b[i];
            if val >= 5 || val <= - 5 {
                change = true;
                if i + 1 >= b.len() {
                    b.push(0);
                }
                b[i + 1] += val / 5;
                b[i] = val % 5;
            }

            if b[i].abs() > 2 {
                change = true;
                let sign = if b[i] > 0 { 1 } else { -1 };
                b[i + 1] += sign;
                b[i] = (5 - b[i].abs()) * sign * -1;
            }
        }
        if !change { break; }
    }
    b
}

fn to_string(a: &RawRepresentation) -> String {
    let mut res = Vec::<char>::new();
    for val in a {
        res.push(match val {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => { panic!("Representation not normal"); }
        });
    }
    res.iter().rev().collect::<String>()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut numbers = Vec::<RawRepresentation>::new();

    for line_res in lines {
        if let Ok(line) = line_res {
            numbers.push(parse(&line));
        }
    }

    let mut sum = RawRepresentation::new();
    for n in &numbers {
        sum = add_raw(&sum, n);
    }
    let normal = normalise(&sum);
    println!("res {}", to_string(&normal));
}
