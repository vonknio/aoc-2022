use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};

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

fn can_reach_zero(p: Point3D, limits: Point3D, processed: &mut HashSet<Point3D>, droplets: &Vec<Point3D>, cache: &mut HashMap<Point3D, bool>) -> bool {
    if processed.contains(&p) || droplets.contains(&p) {
        return false;
    }

    if p.0 == 0 || p.1 == 0 || p.2 == 0 || p.0 == limits.0 || p.1 == limits.1 || p.2 == limits.2 {
        return true;
    }

    if cache.contains_key(&p) {
        return cache[&p];
    }

    processed.insert(p.clone());
    let neighbours = [
        (p.0 + 1, p.1, p.2), (p.0 - 1, p.1, p.2),
        (p.0, p.1 + 1, p.2), (p.0, p.1 - 1, p.2),
        (p.0, p.1, p.2 + 1), (p.0, p.1, p.2 - 1)
    ];
    let res = neighbours.iter().any(|n| can_reach_zero(*n, limits, processed, droplets, cache));
    cache.insert(p.clone(), res);
    res
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

    let max_x = points.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = points.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let max_z = points.iter().max_by(|a, b| a.2.cmp(&b.2)).unwrap().2;

    let mut trapped_air: Vec<Point3D> = Vec::new();
    for x in 0..max_x {
        for y in 0..max_y {
            for z in 0..max_z {
                if !points.contains(&(x, y, z)) && !can_reach_zero((x, y, z), (max_x, max_y, max_z), &mut HashSet::new(), &points, &mut HashMap::new()) {
                    trapped_air.push((x, y, z));
                }
            }
        }
    }
    let air_grouping0 = group(&trapped_air, 0);
    let air_grouping1 = group(&trapped_air, 1);
    let air_grouping2 = group(&trapped_air, 2);
    let air_count = count_touching_faces(&air_grouping0) + count_touching_faces(&air_grouping1) + count_touching_faces(&air_grouping2);
    let air_faces = (trapped_air.len() * 6) as i32 - air_count;

    let exterior_faces = actual_faces - air_faces;

    println!("exterior_faces {exterior_faces}");
}
