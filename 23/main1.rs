use std::fs::File;
use std::path::Path;
use std::env;
use std::collections::{HashMap, HashSet};
use std::io::{BufReader, BufRead};
use std::cmp::{min, max};

#[derive(Debug)]
enum Direction {
    North, South, West, East
}
type Point = (i32, i32);
type Board = HashSet<Point>;
type Suggestions = HashMap<Point, Vec<Point>>;

impl Direction {
    fn is_ok(&self, board: &Board, elf: Point) -> bool {
        match self {
            Direction::North =>
                !board.contains(&(elf.0 - 1, elf.1 - 1)) &&
                    !board.contains(&(elf.0 - 1, elf.1)) &&
                        !board.contains(&(elf.0 - 1, elf.1 + 1)),
            Direction::South =>
                !board.contains(&(elf.0 + 1, elf.1 - 1)) &&
                    !board.contains(&(elf.0 + 1, elf.1)) &&
                        !board.contains(&(elf.0 + 1, elf.1 + 1)),
            Direction::West =>
                !board.contains(&(elf.0 + 1, elf.1 - 1)) &&
                    !board.contains(&(elf.0, elf.1 - 1)) &&
                        !board.contains(&(elf.0 - 1, elf.1 - 1)),
            Direction::East =>
                !board.contains(&(elf.0 + 1, elf.1 + 1)) &&
                    !board.contains(&(elf.0, elf.1 + 1)) &&
                        !board.contains(&(elf.0 - 1, elf.1 + 1)),
        }
    }

    fn step(&self, elf: Point) -> Point {
        match self {
            Direction::North => (elf.0 - 1, elf.1),
            Direction::South => (elf.0 + 1, elf.1),
            Direction::West => (elf.0, elf.1 - 1),
            Direction::East => (elf.0, elf.1 + 1),
        }
    }
}

fn suggest(board: &Board, elf: Point, start_dir: usize) -> Point {
    let dirs = [Direction::North, Direction::South, Direction::West, Direction::East];

    if Direction::North.is_ok(board, elf) && Direction::South.is_ok(board, elf) &&
                Direction::West.is_ok(board, elf) && Direction::East.is_ok(board, elf) {
        return elf;
    }

    for i in 0..4 {
        let dir = &dirs[(start_dir + i) % 4];
        if dir.is_ok(board, elf) {
            return dir.step(elf);
        }
    }
    elf
}

fn board_dimensions(board: &Board) -> (i32, i32) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    for (x, y) in board {
        min_x = min(min_x, *x);
        min_y = min(min_y, *y);
        max_x = max(max_x, *x);
        max_y = max(max_y, *y);
    }
    (max_x - min_x + 1, max_y - min_y + 1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut board = Board::new();
    let mut row = 0;
    let mut col;

    for line_res in lines {
        col = 0;
        if let Ok(line) = line_res {
            for c in line.chars() {
                if c == '#' {
                    board.insert((row, col));
                }
                col += 1;
            }
        }
        row += 1;
    }

    let round_no = 10;
    let mut start_dir = 0;
    for _round in 0..round_no {
        // First half
        let mut suggestions = Suggestions::new();
        for elf in &board {
            let suggestion = suggest(&board, *elf, start_dir);
            suggestions.entry(suggestion).and_modify(|v| v.push(*elf)).or_insert(vec![*elf]);
        }

        // Second half
        for (dest, v) in suggestions {
            if v.len() == 1 {
                board.remove(&v[0]);
                board.insert(dest);
            }
        }

        start_dir += 1;
    }

    let (x, y) = board_dimensions(&board);
    let res = (x * y) - board.len() as i32;
    println!("res: {res}");
}
