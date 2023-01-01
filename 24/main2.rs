use std::fs::File;
use std::path::Path;
use std::env;
use std::collections::{VecDeque, HashSet};
use std::io::{BufReader, BufRead};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    North, South, West, East
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Blizzard {
    dir: Direction,
    start_pos: Point,
}

impl Blizzard {
    fn step(&self, n_steps: usize, rows: usize, cols: usize) -> Point {
        self.dir.step(self.start_pos, n_steps, rows, cols)
    }
}

type Point = (usize, usize);
// Current position, number of steps
type State = (Point, usize);
// Current position, number of steps % rows - 2, number of steps % cols - 2
type MemoizedState = (Point, usize, usize);

impl Direction {
    fn step(&self, p: Point, n_steps: usize, rows: usize, cols: usize) -> Point {
        match self {
            Direction::North => (1 + (p.0 - 1 + n_steps * (rows - 2) - n_steps) % (rows - 2), p.1),
            Direction::South => (1 + (p.0 + n_steps - 1) % (rows - 2), p.1),
            Direction::West => (p.0, 1 + (p.1 + n_steps * (cols - 2) - n_steps - 1) % (cols - 2)),
            Direction::East => (p.0, 1 + (p.1 + n_steps - 1) % (cols - 2))
        }
    }
}

fn is_wall(p: Point, rows: usize, cols: usize) -> bool {
    (p.0 == 0 && p.1 != 1) || (p.1 == 0) || (p.1 == cols - 1) || (p.0 == rows - 1 && p.1 != cols - 2)
}

fn find_way(start: Point, end: Point, blizzards: &Vec<Blizzard>, init_steps: usize, rows: usize, cols: usize) -> usize {
    let mut queue = VecDeque::<State>::new();
    queue.push_back((start, init_steps));
    let mut memoized = HashSet::<MemoizedState>::new();

    loop {
        let (p, steps) = queue.pop_front().unwrap();
        let mod_row = steps % (rows - 2);
        let mod_col = steps % (cols - 2);
        let memoized_state = (p, mod_row, mod_col);
        if memoized.contains(&memoized_state) {
            continue;
        }
        memoized.insert(memoized_state);

        if p == end {
            return steps;
        }

        let mut cur_free = true;
        let mut west_free = p.1 > 0 && !is_wall((p.0, p.1 - 1), rows, cols);
        let mut north_free = p.0 > 0 && !is_wall((p.0 - 1, p.1), rows, cols);
        let mut south_free = !is_wall((p.0 + 1, p.1), rows, cols);
        let mut east_free = !is_wall((p.0, p.1 + 1), rows, cols);
        for bl in blizzards {
            let new_bl_pos = bl.step(steps + 1, rows, cols);
            if new_bl_pos == p { cur_free = false; }
            if p.0 > 0 && new_bl_pos == (p.0 - 1, p.1) { north_free = false; }
            if new_bl_pos == (p.0 + 1, p.1) { south_free = false; }
            if p.1 > 0 && new_bl_pos == (p.0, p.1 - 1) { west_free = false; }
            if new_bl_pos == (p.0, p.1 + 1) { east_free = false; }
        }

        if north_free {
            let new_p = (p.0 - 1, p.1);
            queue.push_back((new_p, steps + 1));
        }

        if west_free {
            let new_p = (p.0, p.1 - 1);
            queue.push_back((new_p, steps + 1));
        }

        if south_free {
            let new_p = (p.0 + 1, p.1);
            queue.push_back((new_p, steps + 1));
        }

        if east_free {
            let new_p = (p.0, p.1 + 1);
            queue.push_back((new_p, steps + 1));
        }

        // Stay in place
        if cur_free {
            queue.push_back((p, steps + 1));
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut blizzards = Vec::<Blizzard>::new();
    let mut row = 0;
    let mut col = 0;

    for line_res in lines {
        col = 0;
        if let Ok(line) = line_res {
            for c in line.chars() {
                if c == '#' || c == '.' {

                } else {
                    let dir = match c {
                        '^' => Direction::North,
                        'v' => Direction::South,
                        '>' => Direction::East,
                        '<' => Direction::West,
                        _ => { panic!("wrong input"); }
                    };
                    blizzards.push(Blizzard {
                        dir: dir,
                        start_pos: (row, col),
                    });
                }
                col += 1;
            }
        }
        row += 1;
    }

    let start = (0, 1);
    let end = (row - 1, col - 2);
    let res1 = find_way(start, end, &blizzards, 0, row, col);
    println!("got there in {res1}");
    let res2 = find_way(end, start, &blizzards, res1, row, col);
    println!("got back in {}", res2 - res1);
    let res3 = find_way(start, end, &blizzards, res2, row, col);
    println!("got there again in {}", res3 - res2);
    println!("res: {}", res3);
}
