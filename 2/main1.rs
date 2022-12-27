use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use std::env;

const BASE_VAL_PLAYER: u8 = 'X' as u8;
const ALPHABET: u8 = BASE_VAL_PLAYER - ('A' as u8);

fn move_score(val: u8) -> u8 {
    val - BASE_VAL_PLAYER + 1
}

fn round_score(opponent: u8, player: u8) -> u8 {
    let player = player - ALPHABET;
    if opponent == player {
        3
    } else if player == opponent + 1 || player == opponent - 2 {
        6
    } else {
        0
    }
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
                sum += (move_score(line[2]) + round_score(line[0], line[2])) as u32;
            }
        }
    }

    println!("{sum}");
}
