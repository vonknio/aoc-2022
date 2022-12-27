use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, BufRead};

type Point = (usize, usize);
type Grid = Vec<Vec<u32>>;

// Note: relying on integer overflow for values < 0
fn point_outside_grid(grid: &Grid, p: Point) -> bool {
    p.0 >= grid.len() || p.1 >= grid[0].len()
}

// Calculate visibility for the whole grid from one side
// Returns a grid where each tree is marked with 1 if visible, with 0 otherwise
fn one_side_visibility<F, G>(grid: &Grid, start: Point, get_next_row: F, get_next_el: G) -> Grid
where F: Fn(Point) -> Point, G: Fn(Point) -> Point {
    let mut output: Grid = vec![vec![0; grid[0].len()]; grid.len()];
    let mut cur_row_start = start;
    loop {
        let mut cur_el = cur_row_start;
        let mut prev_val = 0;
        loop {
            if cur_el == cur_row_start || grid[cur_el.0][cur_el.1] > prev_val {
                output[cur_el.0][cur_el.1] = 1;
                prev_val = grid[cur_el.0][cur_el.1];
            }

            cur_el = get_next_el(cur_el);
            if point_outside_grid(grid, cur_el) {
                break;
            }
        }

        cur_row_start = get_next_row(cur_row_start);
        if point_outside_grid(grid, cur_row_start) {
            break;
        }
    }
    return output;
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

    let out_grids = vec![
        one_side_visibility(&grid, (0, 0), |(x, y)| (x, y + 1), |(i, j)| (i + 1, j)),
        one_side_visibility(&grid, (0, grid[0].len() - 1), |(x, y)| (x + 1, y), |(i, j)| (i, j - 1)),
        one_side_visibility(&grid, (grid.len() - 1, 0), |(x, y)| (x, y + 1), |(i, j)| (i - 1, j)),
        one_side_visibility(&grid, (0, 0), |(x, y)| (x + 1, y), |(i, j)| (i, j + 1))
    ];

    let mut res: Vec<u32> = vec![0; grid.len() * grid[0].len()];
    for grid in out_grids.iter().map(|g| g.concat()) {
        for (i, x) in grid.iter().enumerate() {
            res[i] |= x;
        }
    }
    println!("{}", res.iter().sum::<u32>());
}
