use std::fs::File;
use std::path::Path;
use std::io::Read;

fn main() {
    let path = Path::new("input1.txt");
    let mut file = File::open(&path).expect("Couldn't open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read file.");

    let mut sums: Vec<i32> = Vec::new();
    let elf_lists = contents.split("\n\n");
    for list in elf_lists {
        let elf_items = list.split("\n");
        let mut sum = 0;
        for item in elf_items {
            let integer_item: i32 = item.parse().unwrap_or(0);
            sum += integer_item;
        }
        sums.push(sum);
    }

    sums.sort_by(|a, b| b.cmp(a));
    println!("{}", sums[0] + sums[1] + sums[2]);
}
