use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

type Point2D = (i32, i32);
type Point3D = (i32, i32, i32);
type Grouping = HashMap<Point2D, Vec<i32>>;

fn group(points: &Vec<Point3D>, exclude_coord: u32) -> Grouping {
    let mut grouping = Grouping::new();
    for (x, y, z) in points {
        match exclude_coord {
            0 => {
                grouping.entry((*y, *z)).and_modify(|v| v.push(*x)).or_insert(vec![*x]);
            },
            1 => {
                grouping.entry((*x, *z)).and_modify(|v| v.push(*y)).or_insert(vec![*y]);
            },
            2 => {
                grouping.entry((*x, *y)).and_modify(|v| v.push(*z)).or_insert(vec![*z]);
            },
            _ => { panic!("..."); }
        }
    }
    return grouping;
}

// If points are touching, they have 2 touching faces
fn count_touching_faces(grouping: &Grouping) -> i32 {
    let mut sum = 0;
    for (_, v) in grouping {
        let mut v = v.clone();
        v.sort();
        let mut prev = -2;
        for x in v {
            if x == prev + 1 {
                sum += 2;
            }
            prev = x;
        }
    }
    return sum;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut points: Vec<Point3D> = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            let (x, rest) = line.split_once(",").unwrap();
            let (y, z) = rest.split_once(",").unwrap();
            points.push((x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()));
        }
    }

    let grouping0 = group(&points, 0);
    let grouping1 = group(&points, 1);
    let grouping2 = group(&points, 2);

    let count0 = count_touching_faces(&grouping0);
    let count1 = count_touching_faces(&grouping1);
    let count2 = count_touching_faces(&grouping2);

    let max_faces = (points.len() * 6) as i32;
    let actual_faces = max_faces - count0 - count1 - count2;
    println!("{actual_faces}");
}
