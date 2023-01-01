use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, BufRead};
use std::cmp::{max, min};

#[derive(Copy, Clone, Debug)]
enum Orientation { Left, Right, Down, Up }

#[derive(Copy, Clone, PartialEq, Debug)]
enum Turn { Left, Right, Stop }

#[derive(Copy, Clone, PartialEq, Debug)]
enum Tile { Void, Grass, Wall }

type Point = (usize, usize);
type Instruction = (usize, Turn);
type Board = Vec<Vec<Tile>>;
type State = (Point, Orientation, usize);

struct Cube {
    n: usize,
    sides: Vec<Side>,
}

#[derive(Clone, Debug)]
struct Side {
    id: usize,
    board: Board,
    start_coord: Point,
}

// The transitions are hardcoded for my custom input
impl Side {
    // Leave the side when moving downwards from point p. Returns the new point on side.bottom
    fn leave(&self, p: Point, orientation: Orientation) -> (Point, usize, Orientation) {
        let n = self.board.len();
        match orientation {
            Orientation::Right => {
                // Assuming that the column is n-1
                match self.id {
                    1 => ((p.0, 0), 2,  Orientation::Right),
                    2 => ((n - 1 - p.0, n - 1), 5, Orientation::Left),
                    3 => ((n - 1, p.0), 2, Orientation::Up),
                    4 => ((p.0, 0), 5, Orientation::Right),
                    5 => ((n - 1 - p.0, n - 1), 2, Orientation::Left),
                    6 => ((n - 1, p.0), 5, Orientation::Up),
                    _ => { panic!("..."); }
                }
            },
            Orientation::Left => {
                // Assuming that the column is 0
                match self.id {
                    1 => ((n - 1 - p.0, 0), 4, Orientation::Right),
                    2 => ((p.0, n - 1), 1, Orientation::Left),
                    3 => ((0, p.0), 4, Orientation::Down),
                    4 => ((n - 1 - p.0, 0), 1, Orientation::Right),
                    5 => ((p.0, n - 1), 4, Orientation::Left),
                    6 => ((0, p.0), 1, Orientation::Down),
                    _ => { panic!("..."); }
                }
            },
            Orientation::Up => {
                // Assuming the row is 0
                match self.id {
                    1 => ((p.1, 0), 6, Orientation::Right),
                    2 => ((n - 1, p.1), 6, Orientation::Up),
                    3 => ((n - 1, p.1), 1, Orientation::Up),
                    4 => ((p.1, 0), 3, Orientation::Right),
                    5 => ((n - 1, p.1), 3, Orientation::Up),
                    6 => ((n - 1, p.1), 4, Orientation::Up),
                    _ => { panic!("..."); }
                }
            },
            Orientation::Down => {
                // Assuming the row is n - 1
                match self.id {
                    1 => ((0, p.1), 3, Orientation::Down),
                    2 => ((p.1, n - 1), 3, Orientation::Left),
                    3 => ((0, p.1), 5, Orientation::Down),
                    4 => ((0, p.1), 6, Orientation::Down),
                    5 => ((p.1, n - 1), 6, Orientation::Left),
                    6 => ((0, p.1), 2, Orientation::Down),
                    _ => { panic!("..."); }
                }
            },
        }
    }
}

impl Orientation {
    fn step(&self, p: Point, n: usize) -> Option<Point> {
        match self {
            Orientation::Left => {
                if p.1 > 0 {
                    Some((p.0, p.1 - 1))
                } else { None }
            },
            Orientation::Right => {
                if p.1 + 1 < n {
                    Some((p.0, p.1 + 1))
                } else { None }
            },
            Orientation::Down => {
                if p.0 + 1 < n {
                    Some((p.0 + 1, p.1))
                } else { None }
            },
            Orientation::Up => {
                if p.0 > 0 {
                    Some((p.0 - 1, p.1))
                } else { None }
            },
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

impl Cube {
    fn fold_from_board(board: &Board, n: usize) -> Cube {
        let mut side_foldout = vec![vec![0; 4]; 4];
        let mut k = 1;
        for i in 0..4 {
            for j in 0..4 {
                if i * n < board.len() && j * n < board[0].len() && board[i * n][j * n] != Tile::Void {
                    side_foldout[i][j] = k;
                    k += 1;
                }
            }
        }

        let mut sides = vec![
            Side {
                id: 1,
                board: Board::new(),
                start_coord: (0, 0),
            },
            Side {
                id: 2,
                board: Board::new(),
                start_coord: (0, 0),
            },
            Side {
                id: 3,
                board: Board::new(),
                start_coord: (0, 0),
            },
            Side {
                id: 4,
                board: Board::new(),
                start_coord: (0, 0),
            },
            Side {
                id: 5,
                board: Board::new(),
                start_coord: (0, 0),
            },
            Side {
                id: 6,
                board: Board::new(),
                start_coord: (0, 0),
            },
        ];

        let mut k = 0;
        for i in 0..4 {
            for j in 0..4 {
                if i * n < board.len() && j * n < board[0].len() && board[i * n][j * n] != Tile::Void {
                    sides[k].start_coord = (i * n, j * n);
                    for row in i * n .. (i * n + n) {
                        sides[k].board.push(Vec::new());
                        for col in j * n .. (j * n + n) {
                            sides[k].board[row - i * n].push(board[row][col]);
                        }
                    }
                    k += 1;
                }
            }
        }

        Cube {
            n: n,
            sides: sides,
        }
    }
}

fn follow_instruction(cube: &Cube, start_state: State, instr: Instruction) -> (State, bool) {
    let (p, orientation, side) = start_state;
    let (k, t) = instr;
    let mut new_p = p;
    let mut new_side = side;
    let mut new_orientation = orientation;
    for _ in 0..k {
        let new_p_cand = new_orientation.step(new_p, cube.n);
        let mut new_side_cand = new_side;
        let mut new_orientation_cand = new_orientation;
        let new_p_cand = match new_p_cand {
            Some(new_p_cand) => new_p_cand,
            None => {
                let (new_p_cand_new_board, s, or) = cube.sides[side - 1].leave(new_p, new_orientation);
                new_side_cand = s;
                new_orientation_cand = or;
                new_p_cand_new_board
            }
        };

        match cube.sides[new_side_cand - 1].board[new_p_cand.0][new_p_cand.1] {
            Tile::Wall => {
                break;
            },
            Tile::Void => {
                panic!("This shouldn't happen.")
            },
            Tile::Grass => {
                new_side = new_side_cand;
                new_orientation = new_orientation_cand;
                new_p = new_p_cand;
            }
        }
    }

    let new_state = (new_p, new_orientation.turn(t), new_side);
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
    println!("The board is {} by {}.", board.len(), board[0].len());
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
    let mut n = usize::MAX;

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
            for c in chars {
                row.push(Tile::from(c));
            }
            let n_0 = row.iter().position(|x| *x != Tile::Void).unwrap();
            let n_1 = row.iter().rposition(|x| *x != Tile::Void).unwrap();
            n = min(n_1 - n_0 + 1, n);

            max_width = max(max_width, row.len());
            board.push(row);
        }
    }

    for row in &mut board {
        for _ in row.len()..max_width {
            row.push(Tile::Void);
        }
    }


    let cube = Cube::fold_from_board(&board, n);

    let mut y = 0;
    loop {
        if cube.sides[0].board[0][y] == Tile::Grass {
            break;
        }
        y += 1;
    }

    //print_board(&board);

    let mut state = ((0, y), Orientation::Right, 1);
    for instr in instructions {
        let (new_state, stop) = follow_instruction(&cube, state, instr);
        state = new_state;
        if stop { break; }
    }

    let ((x, y), orientation, side) = state;
    let (start_x, start_y) = cube.sides[side - 1].start_coord;
    let res = 1000 * (1 + x + start_x) + 4 * (1 + y + start_y) + orientation.score();
    println!("res {res}");
}
