use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet};
use std::cmp::{max, min};

type Point = (i32, i32);

// returns true if the sand item stops
fn fall_one_sand_item(taken: &mut HashSet<Point>) -> bool {
    let mut pos: Point = (500, 0);
    let mut step_count: u16 = 0;
    loop {
        if step_count >= u16::MAX {
            return false;
        }
        step_count += 1;
        let (x, y) = pos;
        if !taken.contains(&(x, y + 1)) {
            pos = (x, y + 1);
        } else if !taken.contains(&(x - 1, y + 1)) {
            pos = (x - 1, y + 1);
        } else if !taken.contains(&(x + 1, y + 1)){
            pos = (x + 1, y + 1);
        } else {
            break;
        }
    }
    taken.insert(pos);
    return true;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut taken: HashSet<Point> = HashSet::new();
    for line in lines {
        if let Ok(line) = line {
            let mut prev_point: Option<Point> = None;
            for p in line.split(" -> ") {
                let (x, y) = p.split_once(",").unwrap();
                let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
                taken.insert((x, y));
                if let Some((a, b)) = prev_point {
                    for i in min(x, a)..max(x, a) {
                        taken.insert((i, y));
                    }
                    for i in min(y, b)..max(y, b) {
                        taken.insert((x, i));
                    }
                    taken.insert((a, b));
                }
                prev_point = Some((x, y));
            }
        }
    }

    let mut count = 0;
    loop {
        if fall_one_sand_item(&mut taken) {
            count += 1;
        } else {
            break;
        }
    }
    println!("{count}");
}
