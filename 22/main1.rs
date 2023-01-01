use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, BufRead};
use std::cmp::max;

#[derive(Copy, Clone, Debug)]
enum Orientation { Left, Right, Down, Up }

#[derive(Copy, Clone, PartialEq, Debug)]
enum Turn { Left, Right, Stop }

#[derive(Copy, Clone, PartialEq, Debug)]
enum Tile { Void, Grass, Wall }

type Point = (usize, usize);
type Instruction = (usize, Turn);
type Board = Vec<Vec<Tile>>;
type State = (Point, Orientation);

impl Orientation {
    fn step(&self, p: Point) -> Point {
        match self {
            Orientation::Left => (p.0, p.1 - 1),
            Orientation::Right => (p.0, p.1 + 1),
            Orientation::Down => (p.0 + 1, p.1),
            Orientation::Up => (p.0 - 1, p.1),
        }
    }

    fn turn(&self, t: Turn) -> Orientation {
        match (self, t) {
            (Orientation::Left, Turn::Left) => Orientation::Down,
            (Orientation::Left, Turn::Right) => Orientation::Up,
            (Orientation::Right, Turn::Left) => Orientation::Up,
            (Orientation::Right, Turn::Right) => Orientation::Down,
            (Orientation::Up, Turn::Left) => Orientation::Left,
            (Orientation::Up, Turn::Right) => Orientation::Right,
            (Orientation::Down, Turn::Left) => Orientation::Right,
            (Orientation::Down, Turn::Right) => Orientation::Left,
            _ => self.clone(),
        }
    }

    fn opposite(&self) -> Orientation {
        self.turn(Turn::Left).turn(Turn::Left)
    }

    fn score(&self) -> usize {
        match self {
            Orientation::Right => 0,
            Orientation::Down => 1,
            Orientation::Left => 2,
            Orientation::Up => 3,
        }
    }
}

impl Turn {
    fn from(c: char) -> Turn {
        match c {
            'L' => Turn::Left,
            'R' => Turn::Right,
            _ => { panic!("Can't parse turn."); }
        }
    }
}

impl Tile {
    fn from(c: char) -> Tile {
        match c {
            '.' => Tile::Grass,
            '#' => Tile::Wall,
            _ => Tile::Void,
        }
    }
}

fn follow_instruction(board: &Board, start_state: State, instr: Instruction) -> (State, bool) {
    let (p, orientation) = start_state;
    let (k, t) = instr;
    let mut new_p = p;
    for _ in 0..k {
        let new_p_cand = orientation.step(new_p);
        match board[new_p_cand.0][new_p_cand.1] {
            Tile::Wall => {
                break;
            },
            Tile::Void => {
                let mut op_dir_p = new_p;
                let op_dir = orientation.opposite();
                // Find the opposite respawn point
                loop {
                    match board[op_dir_p.0][op_dir_p.1] {
                        Tile::Void => { break; }
                        _ => { op_dir_p = op_dir.step(op_dir_p); }
                    }
                }
                op_dir_p = orientation.step(op_dir_p);
                if board[op_dir_p.0][op_dir_p.1] == Tile::Grass {
                    new_p = op_dir_p;
                } else {
                    break;
                }
            },
            Tile::Grass => {
                new_p = new_p_cand;
            }
        }
    }

    let new_state = (new_p, orientation.turn(t));
    return (new_state, t == Turn::Stop)
}

fn print_board(board: &Board) {
    println!("");
    for row in board {
        for el in row {
            match el {
                Tile::Void => { print!("_"); }
                Tile::Grass => { print!("."); }
                Tile::Wall => { print!("#"); }
            }
        }
        println!("");
    }
    println!("");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut board_ready = false;
    let mut board = Board::new();
    let mut instructions = Vec::<Instruction>::new();
    let mut max_width = 0;

    for line_res in lines {
        if let Ok(line) = line_res {
            if line.is_empty() {
                board_ready = true;
                continue;
            }

            let chars: Vec<char> = line.chars().collect();

            if board_ready {
                let mut prev_i = 0;
                let mut i = 0;
                loop {
                    if i >= chars.len() || !chars[i].is_digit(10) {
                        let k: usize = line[prev_i..i].parse().unwrap();
                        prev_i = i + 1;
                        let t = if i >= chars.len() {
                            Turn::Stop
                        } else {
                            Turn::from(chars[i])
                        };
                        instructions.push((k, t));
                        if t == Turn::Stop { break; }
                    }
                    i += 1;
                }
                break;
            }

            // Parse next line of board
            let mut row = Vec::<Tile>::new();
            row.push(Tile::Void);
            for c in chars {
                row.push(Tile::from(c));
            }
            row.push(Tile::Void);
            max_width = max(max_width, row.len());
            board.push(row);
        }
    }

    for row in &mut board {
        for _ in row.len()..max_width {
            row.push(Tile::Void);
        }
    }
    board.insert(0, vec![Tile::Void; max_width]);
    board.push(vec![Tile::Void; max_width]);

    let mut y = 0;
    loop {
        if board[1][y] == Tile::Grass {
            break;
        }
        y += 1;
    }

    //print_board(&board);

    let mut state = ((1, y), Orientation::Right);
    for instr in instructions {
        let (new_state, stop) = follow_instruction(&board, state, instr);
        state = new_state;
        if stop { break; }
    }

    let res = 1000 * state.0.0 + 4 * state.0.1 + state.1.score();
    println!("res {res}");
}
