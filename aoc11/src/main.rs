extern crate regex;
use regex::Regex;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    println!("Hello, world!");
}

fn read_file() -> Vec<String> {
    let file = BufReader::new(File::open("input.txt").unwrap());
    file.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| line.len() > 0)
        .collect()
}
