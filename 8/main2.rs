use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, BufRead};
use std::cmp;

type Point = (usize, usize);
type Grid = Vec<Vec<u32>>;

// Note: relying on integer overflow for values < 0
fn point_outside_grid(grid: &Grid, p: Point) -> bool {
    p.0 >= grid.len() || p.1 >= grid[0].len()
}

fn count_one_direction<F>(grid: &Grid, start: Point, get_next_el: F) -> u32
where F: Fn(Point) -> Point {
    let mut count = 0;
    let mut cur_el = start;
    loop {
        cur_el = get_next_el(cur_el);
        if point_outside_grid(grid, cur_el) {
            break;
        }

        count += 1;
        if grid[cur_el.0][cur_el.1] >= grid[start.0][start.1] {
            break;
        }
    }

    return count;
}

fn count_from_tree(grid: &Grid, tree: Point) -> u32 {
    count_one_direction(grid, tree, |(x, y)| (x + 1, y)) *
    count_one_direction(grid, tree, |(x, y)| (x - 1, y)) *
    count_one_direction(grid, tree, |(x, y)| (x, y + 1)) *
    count_one_direction(grid, tree, |(x, y)| (x, y - 1))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut grid: Grid = Vec::new();
    for line_res in lines {
        if let Ok(line) = line_res {
            grid.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
        }
    }

    let mut max: u32 = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            max = cmp::max(max, count_from_tree(&grid, (x, y)))
        }
    }
    println!("{max}");
}
