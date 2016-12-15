extern crate regex;

use regex::Regex;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;


fn main() {
    let gears = read_file();
    let pass = (0..).find(|&i| can_pass(&gears, i)).expect("Reached end of infinite range without finding output");
    println!("First pass at t={}", pass);
}

fn can_pass(gears: &Vec<(i32, i32, i32)>, time: i32) -> bool {
    gears.iter().all(|&(time_offset, init_pos, gear_size)| (init_pos + time_offset + time) % gear_size == 0)
}

fn read_file() -> Vec<(i32, i32, i32)> {
    let file = BufReader::new(File::open("input.txt").unwrap());
    let line_regex = Regex::new(r"Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).").unwrap();
    file.lines()
        .filter_map(|line| {
            line_regex.captures(line.unwrap().as_ref()).and_then(|cap| {
                let time_offset = cap.at(1).and_then(|s| s.parse::<i32>().ok());
                let init_pos = cap.at(3).and_then(|s| s.parse::<i32>().ok());
                let gear_size = cap.at(2).and_then(|s| s.parse::<i32>().ok());
                match (time_offset, init_pos, gear_size) {
                    (Some(a), Some(b), Some(c)) => Some((a,b,c)),
                    _ => None
                }
            })
        })
        .collect()
}
