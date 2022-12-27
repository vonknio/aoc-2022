use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, BufRead};
use std::collections::{VecDeque, HashSet};

type Point = (usize, usize);
type Grid = Vec<Vec<u32>>;

fn char_to_height(ch: char) -> u32 {
    match ch {
        'S' => char_to_height('a'),
        'E' => char_to_height('z'),
        _ => ch as u32
    }
}

fn point_outside_grid(grid: &Grid, p: Point) -> bool {
    p.0 >= grid.len() || p.1 >= grid[0].len()
}

fn bfs(grid: &Grid, start: Point, goal: Point) -> u32 {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<(Point, u32)> = VecDeque::new();
    queue.push_back((start, 0));
    loop {
        if queue.is_empty() { break; }
        let (p, dist) = queue.pop_front().unwrap();
        if p == goal { return dist; }

        if !visited.contains(&p) {
            visited.insert(p);
            for nbr in [(p.0, p.1 + 1), (p.0, p.1 - 1), (p.0 + 1, p.1), (p.0 - 1, p.1)] {
                if !point_outside_grid(grid, nbr) && grid[p.0][p.1] + 1 >= grid[nbr.0][nbr.1] {
                    queue.push_back((nbr, dist + 1));
                }
            }
        }
    }

    return u32::MAX;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut grid: Grid = Vec::new();
    let mut start: Point = (0, 0);
    let mut goal: Point = (0, 0);
    for (row, line_res) in lines.enumerate() {
        grid.push(Vec::new());
        if let Ok(line) = line_res {
            for (col, ch) in line.chars().enumerate() {
                if ch == 'S' { start = (row, col); }
                else if ch == 'E' { goal = (row, col); }

                grid[row].push(char_to_height(ch));
            }
        }
    }

    println!("{}", bfs(&grid, start, goal));
}
