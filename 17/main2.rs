use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::env;

type Grid = Vec<u8>;

struct Shape {
    // Kept as a 4x4 matrix
    outline: u16,
    width: usize,
}

fn pos_ok(grid: &Grid, shape: &Shape, pos: (usize, usize)) -> bool {
    // Double check because of integer overflow
    if pos.1 > 6 || pos.1 + shape.width > 7 {
        return false;
    }
    if pos.0 > usize::MAX - 100 {
        return false;
    }
    for i in 0..4 {
        for j in 0..shape.width {
            if pos.0 + i < grid.len() && (grid[pos.0 + i] & (1 << (pos.1 + j))) > 0 && shape.outline & (1 << (i * 4 + j)) > 0 {
                return false;
            }
        }
    }
    return true;
}

fn fix_shape_in_pos(grid: &mut Grid, shape: &Shape, pos: (usize, usize), heights: &mut Vec<usize>) {
    for i in 0..4 {
        for j in 0..4 {
            if shape.outline & (1 << (i * 4 + j)) > 0 {
                if pos.0 + i >= grid.len() {
                    grid.push(0);
                }

                grid[pos.0 + i] |= 1 << (pos.1 + j);
                heights[pos.1 + j] = pos.0 + i;
            }
        }
    }
}

fn fall_rock(grid: &mut Grid, shape: &Shape, step: i32, pushes: &Vec<i32>, heights: &mut Vec<usize>) -> i32 {
    let highest_rock = grid.len();
    let mut pos = (highest_rock + 3, 2);
    let mut new_step = step;
    loop {
        let air_pushed_pos = (pos.0, (pos.1 as i32 + pushes[new_step as usize % pushes.len()]) as usize);
        if pos_ok(grid, shape, air_pushed_pos) {
            pos = air_pushed_pos;
    }
        new_step += 1;
        let fall_down_pos = (pos.0 - 1, pos.1);
        if pos_ok(grid, shape, fall_down_pos) {
            pos = fall_down_pos;
        } else {
            break;
        }
    }
    fix_shape_in_pos(grid, shape, pos, heights);

    return new_step;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let mut file = File::open(&path).expect("Couldn't open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read file.");

    let pushes: Vec<i32> = contents.trim_end().chars().map(|x| if x == '>' { 1 } else if x == '<' { -1 } else { panic!("AAAAA") }).collect();
    let shapes = [
        Shape {
            outline: 0b0000000000001111,
            // ####
            width: 4,
        },
        Shape {
            outline: 0b0000001001110010,
                // .#.
                // ###
                // .#.
            width: 3,
        },
        Shape {
            outline: 0b0000010001000111,
                // ..#.
                // ..#.
                // ###.
            width: 3,
        },
        Shape {
            outline: 0b0001000100010001,
                // #
                // #
                // #
                // #
            width: 1,
        },
        Shape {
            outline: 0b0000000000110011,
                // ##
                // ##
            width: 2,
        }
    ];

    let mut step = 0;
    let mut grid: Grid = Vec::new();
    let mut heights: Vec<usize> = vec![0; 7];
    let mut prev_cycle_row: u8 = 0;
    let mut prev_cycle_height: u128 = 0;
    let mut prev_rock: u128 = 0;
    let mut remaining_cycles_gain: u128 = 0;

    let rock_no = 1000000000000;
    let mut rock: usize = 0;
    loop {
        if rock >= rock_no { break; }
        step = fall_rock(&mut grid, &shapes[rock % shapes.len()], step, &pushes, &mut heights);

        // Detect a cycle
        // The fact that this works (on my input) is a coincidence, since the last row at that point has one empty spot
        if rock as usize % shapes.len() == 0 && step as usize % pushes.len() == 0 {
            let row = grid[grid.len() - 1];
            if row == prev_cycle_row {
                let cycle_gain = grid.len() as u128 - prev_cycle_height;
                let window_size: u128 = rock as u128 - prev_rock;
                let remaining: u128 = rock_no as u128 - rock as u128;
                let remaining_cycles: u128 = remaining / window_size;
                remaining_cycles_gain = remaining_cycles * cycle_gain;
                rock += (remaining_cycles * window_size) as usize;
            }
            prev_cycle_row = row;
            prev_rock = rock as u128;
            prev_cycle_height = grid.len() as u128;
        }

        rock += 1;
    }
    println!("{}", remaining_cycles_gain + grid.len() as u128);
}
