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
    pos: Point,
}

impl Blizzard {
    fn step(&mut self, rows: usize, cols: usize) {
        self.pos = self.dir.step(self.pos, rows, cols);
    }
}

type Point = (usize, usize);
type State = (Point, Vec<Blizzard>, usize);
type MemoizedState = (Point, Vec<Blizzard>);

impl Direction {
    fn step(&self, p: Point, rows: usize, cols: usize) -> Point {
        match self {
            Direction::North => if p.0 > 1 { (p.0 - 1, p.1) } else { (rows - 2, p.1) },
            Direction::South => if p.0 < rows - 2 { (p.0 + 1, p.1) } else { (1, p.1) },
            Direction::West => if p.1 > 1 { (p.0, p.1 - 1) } else { (p.0, cols - 2) },
            Direction::East => if p.1 < cols - 2 { (p.0, p.1 + 1) } else { (p.0, 1) },
        }
    }
}

fn is_wall(p: Point, rows: usize, cols: usize) -> bool {
    (p.0 == 0 && p.1 != 1) || (p.1 == 0) || (p.1 == cols - 1) || (p.0 == rows - 1 && p.1 != cols - 2)
}

fn find_way(me: Point, blizzards: &Vec<Blizzard>, rows: usize, cols: usize) -> usize {
    let mut queue = VecDeque::<State>::new();
    queue.push_back((me, blizzards.clone(), 0));
    let mut memoized = HashSet::<MemoizedState>::new();

    loop {
        let (p, blizzards, steps) = queue.pop_front().unwrap();
        let memoized_state = (p, blizzards.clone());
        if memoized.contains(&memoized_state) {
            continue;
        }
        memoized.insert(memoized_state);

        if p == (rows - 1, cols - 2) {
            return steps;
        }

        let mut new_blizzards = blizzards.clone();
        let mut cur_free = true;
        let mut west_free = p.1 > 0 && !is_wall((p.0, p.1 - 1), rows, cols);
        let mut north_free = p.0 > 0 && !is_wall((p.0 - 1, p.1), rows, cols);
        let mut south_free = !is_wall((p.0 + 1, p.1), rows, cols);
        let mut east_free = !is_wall((p.0, p.1 + 1), rows, cols);
        for bl in &mut new_blizzards {
            bl.step(rows, cols);
            if bl.pos == p { cur_free = false; }
            if p.0 > 0 && bl.pos == (p.0 - 1, p.1) { north_free = false; }
            if bl.pos == (p.0 + 1, p.1) { south_free = false; }
            if p.1 > 0 && bl.pos == (p.0, p.1 - 1) { west_free = false; }
            if bl.pos == (p.0, p.1 + 1) { east_free = false; }
        }

        // Go south
        if south_free {
            queue.push_back(((p.0 + 1, p.1), new_blizzards.clone(), steps + 1));
        }

        // Go east
        if east_free {
            queue.push_back(((p.0, p.1 + 1), new_blizzards.clone(), steps + 1));
        }

        // Go north
        if north_free {
            queue.push_back(((p.0 - 1, p.1), new_blizzards.clone(), steps + 1));
        }

        // Go west
        if west_free {
            queue.push_back(((p.0, p.1 - 1), new_blizzards.clone(), steps + 1));
        }

        // Stay in place
        if cur_free {
            queue.push_back((p, new_blizzards.clone(), steps + 1));
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
                        pos: (row, col),
                    });
                }
                col += 1;
            }
        }
        row += 1;
    }

    let res = find_way((0, 1), &blizzards, row, col);
    println!("res: {res}");
}
