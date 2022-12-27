use std::fs::File;
use std::path::Path;
use std::env;
use std::io::Read;

fn all_different(slice: &[u8]) -> bool {
    let mut bit_vec: [u8; 26] = Default::default();
    for ch in slice {
        bit_vec[*ch as usize - 97] = 1;
    }
    return bit_vec.iter().sum::<u8>() == 4
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(args.get(1).expect("No filename provided."));
    let mut file = File::open(&path).expect("Couldn't open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read file.");

    let mut i = 0;
    for slice in contents.as_bytes().windows(4) {
        if all_different(slice) {
            break;
        }
        i += 1;
    }
    println!("{}", i + 4);
}
