use std::io::{Error};
use std::num::ParseIntError;
use std::str::Lines;

fn main() {
    let lines = include_str!("./input.txt")
        .split("\n\n")
        .collect::<Vec<&str>>();
    println!("{}", 1)
}
