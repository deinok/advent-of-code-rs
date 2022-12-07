use std::io::{Error, Lines};
use std::num::ParseIntError;

fn main() {
    let result = include_str!("./input.txt")
        .split("\n\n")
        .map(|x| x.split("\n").filter_map(|y|y.parse::<u32>().ok()).sum::<u32>())
        .max()
        .unwrap();
    println!("{}", result)
}
