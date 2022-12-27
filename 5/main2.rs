use std::fs::File;
use std::path::Path;
use std::env;
use std::io::Read;

// Hardcoded number of stacks because I can
const STACK_NO: usize = 9;
const COL_WIDTH: usize = STACK_NO * 4;
const EMPTY_VEC: Vec<u8> = Vec::new();

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let mut file = File::open(&path).expect("Couldn't open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read file.");

    let (part1, part2) = contents.split_once("\n\n").expect("Invalid input.");
    let mut cols: [Vec<u8>; COL_WIDTH] = [EMPTY_VEC; COL_WIDTH];
    let mut stacks: [Vec<u8>; STACK_NO] = Default::default();

    for (i, x) in part1.as_bytes().iter().enumerate() {
        cols[i % COL_WIDTH].push(*x);
    }

    let idx_pairs = (1..COL_WIDTH).step_by(4).zip(0..STACK_NO);
    for (i, j) in idx_pairs {
        stacks[j] = cols[i].clone().into_iter()
            .skip_while(|x| !(*x as char).is_alphabetic())
            .take_while(|x| (*x as char).is_alphabetic())
            .collect();
        stacks[j].reverse();
    }

    for line in part2.split("\n") {
        // Abuse the fact that there are max 9 stacks. The two penultimate digits in the command
        // are the stack ids, so the prefix is a possibly multi-digit number of elements to move.
        let instr: Vec<u8> = line.as_bytes().iter().copied()
                .filter(|x| (*x as char).is_digit(10)).collect();
        if instr.len() >= 3 {
            let number_to_move: usize = std::str::from_utf8(&instr[0..instr.len() - 2]).unwrap().parse().unwrap();
            let from_stack = &stacks[instr[instr.len() - 2] as usize - 49];
            let mut vals = Vec::<u8>::new();
            vals.resize(number_to_move, 0);
            vals.copy_from_slice(&from_stack[from_stack.len() - number_to_move ..]);

            let to_stack = &mut stacks[instr[instr.len() - 1] as usize - 49];
            for el in vals {
                to_stack.push(el);
            }

            // I've no idea if there's a way to write this while only defining from_stack once
            let from_stack = &mut stacks[instr[instr.len() - 2] as usize - 49];
            from_stack.truncate(from_stack.len() - number_to_move);
        }
    }

    for i in 0..STACK_NO {
        print!("{}", *stacks[i].last().unwrap_or(&0) as char);
    }
    println!("");
}
