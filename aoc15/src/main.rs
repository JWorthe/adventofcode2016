extern crate regex;

use regex::Regex;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;


fn main() {
    let gears = read_file();
    let pass = (0..).find(|&i| can_pass(&gears, i)).unwrap();
    println!("First pass at t={}", pass);
}

fn can_pass(gears: &Vec<(i32, i32)>, time: i32) -> bool {
    for i in 0..gears.len() {
        let (gear_pos, gear_size) = gears[i];
        if (gear_pos + time + i as i32 + 1) % gear_size != 0 {
            return false;
        }
    }
    true
}

fn read_file() -> Vec<(i32, i32)> {
    let file = BufReader::new(File::open("input.txt").unwrap());
    let line_regex = Regex::new(r"Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+).").unwrap();
    file.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| line.len() > 0)
        .map(|line| {
            let cap = line_regex.captures(line.as_ref()).unwrap();
            (cap.at(2).unwrap().parse().unwrap(), cap.at(1).unwrap().parse().unwrap())
        })
        .collect()
}
