use std::fs::File;
use std::time::Instant;
use memmap2::Mmap;

use json_parser::parse_input;

fn main() {
    let start_time = Instant::now();

    let filename = "10M-point-pairs.json";
    let file = File::open(filename).expect("Unable to open file");
    let mmap = unsafe { Mmap::map(&file).unwrap() };

    let mut it = mmap.iter().map(|c| std::char::from_u32(*c as u32).unwrap());
    let sum = parse_input(&mut it);

    println!("Sum: {}", sum);
    let total_time = start_time.elapsed().as_secs();
    println!("Total time: {}s", total_time);
}

