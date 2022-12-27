use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, BufRead};
use std::collections::BTreeSet;
use std::cmp::{max, min};

type Point = (i32, i32);
type Interval = (i32, i32);

fn intersect(x: &Interval, y: &Interval) -> bool {
    x.1 >= y.0 && x.0 <= y.1
}

struct LineCover {
    set: BTreeSet<Interval>,
}

impl LineCover {
    fn new() -> LineCover {
        LineCover { set: BTreeSet::new() }
    }

    fn insert(&mut self, x: Interval) {
        let ints: Vec<Interval> = self.set.iter().copied().filter(|y| intersect(&x, y)).collect();
        let mut mega_int = x;
        for int in ints {
            mega_int.0 = min(mega_int.0, int.0);
            mega_int.1 = max(mega_int.1, int.1);
            self.set.remove(&int);
        }
        self.set.insert(mega_int);
    }

    fn len(&self) -> usize {
        self.set.len()
    }

    fn get_only_uncovered_point(&self) -> i32 {
        let x = self.set.iter().nth(0).unwrap();
        if x.0 > 0 {
            return 0;
        }
        if self.len() != 2 {
            panic!("Only uncovered point is not well-defined.");
        }
        return x.1 + 1;
    }
}

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

    let mut sources: Vec<Point> = Vec::new();
    let mut beacons: Vec<Point> = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            let (s, b) = line.split_once(":").unwrap();
            let (sx, sy) = s.split_once(", ").unwrap();
            let (sx, sy) = (parse_number(&sx), parse_number(&sy));
            let (bx, by) = b.split_once(", ").unwrap();
            let (bx, by) = (parse_number(&bx), parse_number(&by));

            sources.push((sx, sy));
            beacons.push((bx, by));

        }
    }

    let limit = 4000000;
    for y in 0..limit + 1 {
        let mut lc = LineCover::new();
        for (s, b) in sources.iter().zip(&beacons) {
            if let Some(int) = inaccessible(*s, *b, y) {
                lc.insert(int);
            }
        }
        if lc.len() > 1 {
            let x = lc.get_only_uncovered_point();
            println!("freq is {}", x as i128 * 4000000 + y as i128);
            break;
        }
    }
}
