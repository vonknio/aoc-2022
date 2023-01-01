use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufReader, BufRead};
use std::cmp::{max};

#[derive(Debug, Copy, Clone)]
struct Blueprint {
    ore: i32,
    clay: i32,
    obsidian: (i32, i32),
    geode: (i32, i32),
}

#[derive(Debug, Copy, Clone)]
struct Resources {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl Resources {
    fn incr_with_robots(&self, robots: &Resources) -> Resources {
        Resources {
            ore: self.ore + robots.ore,
            clay: self.clay + robots.clay,
            obsidian: self.obsidian + robots.obsidian,
            geode: self.geode + robots.geode,
        }
    }

    fn new() -> Resources {
        Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

fn parse_number_at_end(a: &str) -> i32 {
    let mut l = a.len() - 1;
    loop {
        if a.chars().nth(l).unwrap().is_digit(10) || a.chars().nth(l).unwrap() == '-' {
            l -= 1
        } else {
            break;
        }
    }
    a[l+1..].parse().unwrap()
}

fn bruteforce(bl: &Blueprint, time: i32, resources: &Resources, robots: &Resources) -> i32 {
    let mut opt = resources.geode;
    if time == 0 { return opt; }

    // Buy geode robot
    if resources.ore >= bl.geode.0 && resources.obsidian >= bl.geode.1 {
        opt = max(opt, bruteforce(
            bl, time - 1,
            &Resources{ ore: resources.ore - bl.geode.0, obsidian: resources.obsidian - bl.geode.1, ..*resources }.incr_with_robots(robots),
            &Resources{ geode: robots.geode + 1, ..*robots })
        );
    } else {
        // Buy obsidian robot
        if resources.ore >= bl.obsidian.0 && resources.clay >= bl.obsidian.1 {
            opt = max(opt, bruteforce(
                bl, time - 1,
                &Resources{ ore: resources.ore - bl.obsidian.0, clay: resources.clay - bl.obsidian.1, ..*resources }.incr_with_robots(robots),
                &Resources{ obsidian: robots.obsidian + 1, ..*robots })
            );
        } else {
            // Buy clay robot
            if resources.ore >= bl.clay {
                opt = max(opt, bruteforce(
                    bl, time - 1,
                    &Resources{ ore: resources.ore - bl.clay, ..*resources }.incr_with_robots(robots),
                    &Resources{ clay: robots.clay + 1, ..*robots })
                );
            }

            // Buy ore robot
            if resources.ore >= bl.ore {
                opt = max(opt, bruteforce(
                    bl, time - 1,
                    &Resources{ ore: resources.ore - bl.ore, ..*resources }.incr_with_robots(robots),
                    &Resources{ ore: robots.ore + 1, ..*robots })
                );
            }

            // Don't buy anything
            opt = max(opt, bruteforce(bl, time - 1, &resources.incr_with_robots(robots), robots));
        }
    }
    return opt;
}

fn blueprint_score(bl: &Blueprint) -> i32 {
    let res = bruteforce(bl, 24, &Resources::new(), &Resources { ore: 1, ..Resources::new() });
    res
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let file = File::open(&path).expect("Couldn't open file.");
    let lines = BufReader::new(file).lines();

    let mut blueprints = Vec::<Blueprint>::new();
    for line in lines {
        if let Ok(line) = line {
            let (ore, rest) = line.split_once(" ore.").unwrap();
            let ore = parse_number_at_end(ore);
            let (clay, rest) = rest.split_once(" ore.").unwrap();
            let clay = parse_number_at_end(clay);
            let (obs_ore, rest) = rest.split_once(" ore and").unwrap();
            let obs_ore = parse_number_at_end(obs_ore);
            let (obs_clay, rest) = rest.split_once(" clay.").unwrap();
            let obs_clay = parse_number_at_end(obs_clay);
            let (geode_ore, rest) = rest.split_once(" ore and").unwrap();
            let geode_ore = parse_number_at_end(geode_ore);
            let (geode_obs, _) = rest.split_once(" obsidian.").unwrap();
            let geode_obs = parse_number_at_end(geode_obs);
            let blueprint = Blueprint {
                ore: ore,
                clay: clay,
                obsidian: (obs_ore, obs_clay),
                geode: (geode_ore, geode_obs),
            };
            //println!("{:#?}", blueprint);
            blueprints.push(blueprint);
        }
    }

    let mut sum = 0;
    for i in 0..blueprints.len() {
        sum += blueprint_score(&blueprints[i]) * (i + 1) as i32;
        println!("done {} / {}", i+1, blueprints.len());
    }
    println!("sum {sum}");
}
