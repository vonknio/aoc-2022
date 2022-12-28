use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::env;

type Grid = Vec<Vec<bool>>;

struct Shape {
    // Kept as a 4x4 matrix
    outline: Vec<Vec<bool>>,
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
            if pos.0 + i < grid.len() && grid[pos.0 + i][pos.1 + j] && shape.outline[3 - i][j] {
                return false;
            }
        }
    }
    return true;
}

fn fix_shape_in_pos(grid: &mut Grid, shape: &Shape, pos: (usize, usize)) {
    for i in 0..4 {
        for j in 0..4 {
            if shape.outline[3 - i][j] {
                if pos.0 + i >= grid.len() {
                    grid.push(vec![false; 7]);
                }
                grid[pos.0 + i][pos.1 + j] = true;
            }
        }
    }
}

fn fall_rock(grid: &mut Grid, shape: &Shape, step: i32, pushes: &Vec<i32>) -> i32 {
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

    fix_shape_in_pos(grid, shape, pos);
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
            outline: vec![
                // ####
                vec![false, false, false, false],
                vec![false, false, false, false],
                vec![false, false, false, false],
                vec![true, true, true, true],
            ],
            width: 4,
        },
        Shape {
            outline: vec![
                // .#.
                // ###
                // .#.
                vec![false, false, false, false],
                vec![false, true, false, false],
                vec![true, true, true, false],
                vec![false, true, false, false],
            ],
            width: 3,
        },
        Shape {
            outline: vec![
                // ..#
                // ..#
                // ###
                vec![false, false, false, false],
                vec![false, false, true, false],
                vec![false, false, true, false],
                vec![true, true, true, false],
            ],
            width: 3,
        },
        Shape {
            outline: vec![
                // #
                // #
                // #
                // #
                vec![true, false, false, false],
                vec![true, false, false, false],
                vec![true, false, false, false],
                vec![true, false, false, false],
            ],
            width: 1,
        },
        Shape {
            outline: vec![
                // ##
                // ##
                vec![false, false, false, false],
                vec![false, false, false, false],
                vec![true, true, false, false],
                vec![true, true, false, false],
            ],
            width: 2,
        }
    ];

    let mut step = 0;
    let mut grid: Grid = Vec::new();

    let rock_no = 2022;
    for rock in 0..rock_no {
        step = fall_rock(&mut grid, &shapes[rock % shapes.len()], step, &pushes);
    }
    println!("{}", grid.len());
}
