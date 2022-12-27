use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, BufRead};

type Grid = Vec<Vec<char>>;

fn draw_pixel(grid: &mut Grid, counter: i32, register: i32) {
    let x = counter as usize / grid[0].len();
    let y = counter as usize % grid[0].len();
    if register == y as i32 || register == y as i32 - 1 || register == y as i32 + 1 {
        grid[x][y] = '#';
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut counter: i32 = 0;
    let mut register: i32 = 1;
    let mut grid: Grid = vec![vec!['.'; 40]; 6];
    for line_res in lines {
        if let Ok(line) = line_res {
            draw_pixel(&mut grid, counter, register);
            counter += 1;

            if line != "noop" {
                draw_pixel(&mut grid, counter, register);
                counter += 1;

                let (_, x) = line.split_once(" ").unwrap();
                let x: i32 = x.parse().unwrap();
                register += x;
            }
        }
    }

    for row in &grid {
        for y in row {
            print!("{y}");
        }
        println!("");
    }
}
