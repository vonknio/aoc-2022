use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use std::cmp::{max, min};

type Point = (i32, i32);

fn parse_number(a: &str) -> i32 {
    let mut l = a.len() - 1;
    loop {
        if a.chars().nth(l).unwrap().is_digit(10) || a.chars().nth(l).unwrap() == '-' {
            l -= 1
        } else {
            break;
        }
    }
    a[l+1..].parse().unwrap()
}

// Returns (min_x, max_x) for row y, None if not blocking anything
fn inaccessible(s: Point, b: Point, y: i32) -> Option<(i32, i32)> {
    let dist = (s.0 - b.0).abs() + (s.1 - b.1).abs();
    let y_dist = (s.1 - y).abs();
    let rem = dist - y_dist;
    if rem >= 0 {
        Some((s.0 - rem, s.0 + rem))
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let y: i32 = 2000000;
    let mut balises_problematiques: HashSet<i32> = HashSet::new();
    for line in lines {
        if let Ok(line) = line {
            let (s, b) = line.split_once(":").unwrap();
            let (sx, sy) = s.split_once(", ").unwrap();
            let (sx, sy) = (parse_number(&sx), parse_number(&sy));
            let (bx, by) = b.split_once(", ").unwrap();
            let (bx, by) = (parse_number(&bx), parse_number(&by));
            if by == y {
                balises_problematiques.insert(bx);
            }
            if let Some((x1, x2)) = inaccessible((sx, sy), (bx, by), y) {
                min_x = min(min_x, x1);
                max_x = max(max_x, x2);
            }
        }
    }

    let b_count = balises_problematiques.iter().filter(|&x| *x >= min_x && *x <= max_x).count() as i32;
    println!("{}", max_x - min_x + 1 - b_count);
}
