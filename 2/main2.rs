use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use std::env;

const BASE_VAL_OPPONENT: u8 = 'A' as u8;
const BASE_VAL_RES: u8 = 'X' as u8;

fn move_score(opponent: u8, res: char) -> u8 {
    let opponent = opponent - BASE_VAL_OPPONENT;
    match res {
        'X' => (opponent + 2) % 3 + 1,
        'Y' => opponent + 1,
        'Z' => (opponent + 1) % 3 + 1,
        _ => panic!("Invalid input"),
    }
}

fn round_score(res: u8) -> u8 {
    (res - BASE_VAL_RES) * 3
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut sum: u32 = 0;
    for line_res in lines {
        if let Ok(line) = line_res {
            let line = line.as_bytes();
            if line.len() >= 3 {
                sum += (move_score(line[0], line[2] as char) + round_score(line[2])) as u32;
            }
        }
    }

    println!("{sum}");
}
