use std::collections::HashSet;
use std::cmp::{max};

// Both Resources and Robots store the values of geode, obsidian, clay, ore in this order as 8-bit chunks
type Resources = u32;
type Robots = u32;

// The blueprint values are hardcoded. The program takes no input.
const ORE_COST: u32 = 4;
const CLAY_COST: u32 = 4;
const OBS_COST_ORE: u32 = 4;
const GEODE_COST_ORE: u32 = 4;

const OBS_COST_CLAY: u32 = 18;
const GEODE_COST_OBS: u32 = 9;

const MAX_COST_ORE: u32 = 4;

#[inline]
fn ore(x: u32) -> u32 {
    (x << 24) >> 24
}

#[inline]
fn clay(x: u32) -> u32 {
    (x << 16) >> 24
}

#[inline]
fn obsidian(x: u32) -> u32 {
    (x << 8) >> 24
}

#[inline]
fn geode(x: u32) -> u32 {
    x >> 24
}

#[inline]
fn add_in_chunks(x: u32, y: u32) -> u32 {
    ((geode(x) + geode(y)) << 24) +
    ((obsidian(x) + obsidian(y)) << 16) +
    ((clay(x) + clay(y)) << 8) +
    (ore(x) + ore(y))
}

#[inline]
fn subtract_in_chunks(x: u32, y: u32) -> u32 {
    ((geode(x) - geode(y)) << 24) +
    ((obsidian(x) - obsidian(y)) << 16) +
    ((clay(x) - clay(y)) << 8) +
    (ore(x) - ore(y))
}

#[inline]
fn add_ore_robot(robots: u32) -> u32 {
    add_in_chunks(robots, 1)
}

#[inline]
fn add_clay_robot(robots: u32) -> u32 {
    add_in_chunks(robots, 1 << 8)
}

#[inline]
fn add_obsidian_robot(robots: u32) -> u32 {
    add_in_chunks(robots, 1 << 16)
}

#[inline]
fn add_geode_robot(robots: u32) -> u32 {
    add_in_chunks(robots, 1 << 24)
}

#[inline]
fn buy_ore_robot(resources: u32, price: u32) -> u32 {
    subtract_in_chunks(resources, price)
}

#[inline]
fn buy_clay_robot(resources: u32, price: u32) -> u32 {
    subtract_in_chunks(resources, price)
}

#[inline]
fn buy_obsidian_robot(resources: u32, price_ore: u32, price_clay: u32) -> u32 {
    subtract_in_chunks(resources, ((price_clay) << 8) + price_ore)
}

#[inline]
fn buy_geode_robot(resources: u32, price_ore: u32, price_obsidian: u32) -> u32 {
    subtract_in_chunks(resources, ((price_obsidian) << 16) + price_ore)
}

type State = (i32, Resources, Robots);

fn bruteforce(time: i32, resources: Resources, robots: Robots, states: &mut HashSet<State>) -> u32 {
    let mut opt = geode(resources);
    if time == 0 { return opt; }

    let state = (time, resources, robots);
    if states.contains(&state) {
        return 0;
    }

   // Buy geode robot
    if ore(resources) >= GEODE_COST_ORE && obsidian(resources) >= GEODE_COST_OBS {
        //println!("buying geode,, before {}, {}, after {}, {}", ore(resources), geode(robots), ore(buy_geode_robot(resources, bl.geode.0, bl.geode.1)), geode(add_geode_robot(robots)));
        opt = max(opt, bruteforce(
            time - 1,
            add_in_chunks(buy_geode_robot(resources, GEODE_COST_ORE, GEODE_COST_OBS), robots),
            add_geode_robot(robots), states),
        );
    } else {
        // Buy obsidian robot
        if obsidian(robots) < GEODE_COST_OBS && ore(resources) >= OBS_COST_ORE && clay(resources) >= OBS_COST_CLAY {
            opt = max(opt, bruteforce(
                time - 1,
                add_in_chunks(buy_obsidian_robot(resources, OBS_COST_ORE, OBS_COST_CLAY), robots),
                add_obsidian_robot(robots), states),
            );
        }

        // Buy clay robot
        if clay(robots) < OBS_COST_CLAY && ore(resources) >= CLAY_COST {
            opt = max(opt, bruteforce(
                time - 1,
                add_in_chunks(buy_clay_robot(resources, CLAY_COST), robots),
                add_clay_robot(robots), states)
            );
        }

        // Buy ore robot
        if ore(robots) < MAX_COST_ORE && ore(resources) >= ORE_COST {
            opt = max(opt, bruteforce(
                time - 1,
                add_in_chunks(buy_ore_robot(resources, ORE_COST), robots),
                add_ore_robot(robots), states),
            );
        }

        // Don't buy anything if you're saving up for something
        // I.e. if you can afford everything, then a purchase needs to be made
        if !(
            ore(resources) >= MAX_COST_ORE &&
            clay(resources) >= OBS_COST_CLAY &&
            obsidian(resources) >= GEODE_COST_OBS
        ) {
            opt = max(opt, bruteforce(time - 1, add_in_chunks(resources, robots), robots, states));
        }
    }
    states.insert(state);
    return opt;
}

fn blueprint_score() -> u32 {
    let res = bruteforce(32, 0, 1, &mut HashSet::new());
    println!(" RES FROM BLUEPRINT {res }");
    res
}

fn main() {
    blueprint_score();
}