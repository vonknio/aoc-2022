use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

type Point = (i32, i32);

fn dist_ok(a: Point, b: Point) -> bool {
    (a.0 - b.0).abs() < 2 && (a.1 - b.1).abs() < 2
}

fn one_step_in_dir(p: Point, dir: char) -> Point {
    match dir {
        'R' => (p.0, p.1 + 1),
        'L' => (p.0, p.1 - 1),
        'U' => (p.0 + 1, p.1),
        'D' => (p.0 - 1, p.1),
        _ => panic!("This shouldn't happen.")
    }
}

// Returns the new position of the tail
// Assumes the distance is too large at the input
fn fix_tail(head: Point, tail: Point) -> Point {
    let (x, y) = tail;
    let guesses = if x != head.0 && y != head.1 {
        [(x + 1, y + 1), (x + 1, y - 1), (x - 1, y - 1), (x - 1, y + 1)]
    } else {
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
    };

    for guess in guesses {
        if dist_ok(head, guess) {
            return guess;
        }
    }
    panic!("This shouldn't happen.");
}

// Returns the new snake and the set of positions visited by the tail in this move
fn move_head(start: &Vec<Point>, dir: char, steps: u32) -> (Vec<Point>, HashSet<Point>) {
    let mut cur = start.clone();
    let mut visited = HashSet::new();
    for _ in 0..steps {
        cur[0] = one_step_in_dir(cur[0], dir);
        for i in 1..start.len() {
            if !dist_ok(cur[i], cur[i - 1]) {
                cur[i] = fix_tail(cur[i - 1], cur[i]);
                if i == start.len() - 1 {
                    visited.insert(cur[i]);
                }
            }
        }
    }
    (cur, visited)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut snake = vec![(0, 0); 10];
    let mut visited: HashSet<Point> = HashSet::from([(0, 0)]);
    for line_res in lines {
        if let Ok(line) = line_res {
            let (dir, steps) = line.split_once(" ").unwrap();
            let dir: char = dir.chars().next().unwrap();
            let steps: u32 = steps.parse().expect("Could't parse the number of steps.");
            let (new_snake, visited_in_move) = move_head(&snake, dir, steps);
            snake = new_snake;
            visited.extend(visited_in_move);
        }
    }

    println!("{}", visited.len());
}
