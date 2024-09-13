use std::fs::File;
use std::io::{BufReader, Read};
use std::time::Instant;

use json_parser::parse_input;

fn main() {
    let start_time = Instant::now();

    let filename = "10-point-pairs.json";
    let file = File::open(filename).expect("Unable to open file");
    let buffered_reader = BufReader::with_capacity(4 * 1 << 20, file); // buffer size = 4MB, doesn't seem to make a difference

    let mut it = buffered_reader.bytes().map(|c| std::char::from_u32(c.unwrap() as u32).unwrap());
    let sum = parse_input(&mut it);

    println!("Sum: {}", sum);
    let total_time = start_time.elapsed().as_secs();
    println!("Total time: {}s", total_time);
}

