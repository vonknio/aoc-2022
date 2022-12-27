use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::env;

fn parse_number(a: &[char]) -> (u32, usize) {
    let l: usize = if a[1].is_digit(10) { 2 } else { 1 };
    let d = a[..l].iter().collect::<String>().parse().unwrap();
    (d, l - 1)
}

// assumes one of the input string starts with [
// returns (..., i, j), where i, j and the amounts of *extra* characters to skip in the main loop
fn compare_list(a: &[char], b: &[char]) -> (Option<bool>, usize, usize) {
    if a[0] != '[' {
        let new_list = if a[1].is_digit(10) { vec!['[', a[0], a[1], ']'] } else { vec!['[', a[0], ']'] };
        let (res, _, j) = compare_list(&new_list, &b);
        return (res, new_list.len() - 3, j);
    }
    if b[0] != '[' {
        let new_list = if b[1].is_digit(10) { vec!['[', b[0], b[1], ']'] } else { vec!['[', b[0], ']'] };
        let (res, i, _) = compare_list(&a, &new_list);
        return (res, i, new_list.len() - 3);
    }

    let mut i: usize = 1;
    let mut j: usize = 1;

    loop {
        if a[i] == ']' && b[j] != ']' {
            return (Some(true), i, j);
        }
        if a[i] != ']' && b[j] == ']' {
            return (Some(false), i, j);
        }
        if a[i] == ']' && b[j] == ']' {
            return (None, i, j);
        }

        if a[i] == ',' { i += 1; }
        if b[j] == ',' { j += 1; }

        if a[i] == '[' || b[j] == '[' {
            let (res, i2, j2) = compare_list(&a[i..], &b[j..]);
            match res {
                Some(_) => return (res, i2, j2),
                None => {
                    i += i2;
                    j += j2;
                }
            }
        } else {
            let (x, i2) = parse_number(&a[i..]);
            let (y, j2) = parse_number(&b[j..]);
            i += i2;
            j += j2;
            if x < y { return (Some(true), i, j); }
            if x > y { return (Some(false), i, j); }
        }

        i += 1;
        j += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let mut file = File::open(&path).expect("Couldn't open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read file.");

    let mut sum: usize = 0;
    let pairs = contents.split("\n\n");
    for (i, pair) in pairs.enumerate() {
        let (a_str, b_str) = pair.split_once("\n").unwrap();
        let a: Vec<char> = a_str.chars().collect();
        let b: Vec<char> = b_str.chars().collect();
        let (res, _, _) = compare_list(&a, &b);
        if let Some(b) = res {
            if b { sum += i + 1; }
        }
    }
    println!("{sum}");
}
