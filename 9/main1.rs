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
fn fix_tail(head: Point, tail: Point, dir: char) -> Point {
    if head.0 != tail.0 && head.1 != tail.1 {
        let (x, y) = tail;
        for guess in [(x + 1, y + 1), (x + 1, y - 1), (x - 1, y - 1), (x - 1, y + 1)] {
            if dist_ok(head, guess) {
                return guess;
            }
        }
    }

    one_step_in_dir(tail, dir)
}

// Returns (new position of the head, new position of the tail, the set of points visited by the tail)
// Assumes that at the input head and tail are at a legal distance
fn move_head(head_start: Point, tail_start: Point, dir: char, steps: u32) -> (Point, Point, HashSet<Point>) {
    let mut head = head_start;
    let mut tail = tail_start;
    let mut visited = HashSet::new();
    for _ in 0..steps {
        head = one_step_in_dir(head, dir);
        if !dist_ok(head, tail) {
            tail = fix_tail(head, tail, dir);
            visited.insert(tail);
        }
    }

    (head, tail, visited)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited: HashSet<Point> = HashSet::from([(0, 0)]);
    for line_res in lines {
        if let Ok(line) = line_res {
            let (dir, steps) = line.split_once(" ").unwrap();
            let dir: char = dir.chars().next().unwrap();
            let steps: u32 = steps.parse().expect("Could't parse the number of steps.");
            let (new_head, new_tail, visited_in_move) = move_head(head, tail, dir, steps);
            head = new_head;
            tail = new_tail;
            visited.extend(visited_in_move);
        }
    }

    println!("{}", visited.len());
}
