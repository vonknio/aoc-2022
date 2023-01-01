use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Number {
    id: usize,
    val: i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut numbers: Vec<Number> = Vec::new();
    let mut id = 0;
    let mut positions: HashMap<usize, usize> = HashMap::new();
    let mut zero_id = 0;
    let mut id_to_val: HashMap<usize, i32> = HashMap::new();
    for line in lines {
        if let Ok(line) = line {
            let val = line.parse().unwrap();
            numbers.push(Number {
                id: id,
                val: val,
            });
            positions.insert(id, id);
            if val == 0 {
                zero_id = id;
            }
            id_to_val.insert(id, val);
            id += 1;
        }
    }

    let n = numbers.len();
    for id in 0..numbers.len() {
        let pos = positions[&id];
        let val = numbers[pos].val;
        let mut i = pos as i32;

        let dir = if val >= 0 { 1 } else { -1 };
        for _ in 0..val.abs() as usize % (n - 1) {
            let i1 = i as usize;
            let i2 = (i + dir + n as i32) as usize % n;
            *positions.entry(numbers[i1].id).or_insert(0) = i2;
            *positions.entry(numbers[i2].id).or_insert(0) = i1;
            let tmp = numbers[i1];
            numbers[i1] = numbers[i2];
            numbers[i2] = tmp;
            i = i2 as i32;
        }
    }

    let zero_pos = positions[&zero_id];
    let i = (zero_pos + 1000) % n;
    let j = (zero_pos + 2000) % n;
    let k = (zero_pos + 3000) % n;
    println!("res {}", numbers[i].val + numbers[j].val + numbers[k].val);
}
