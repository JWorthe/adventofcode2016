use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let lines = read_file();
    let answer_width = lines[0].len();
    for i in 0..answer_width {
        let line = lines.iter().map(|line| line.chars().nth(i).unwrap()).collect::<Vec<_>>();

        let mut char_counts = HashMap::new();
        for character in line {
            *char_counts.entry(character).or_insert(0) += 1;
        }
        let (character, _) = char_counts.iter().min_by_key(|&(_, &count)| count).unwrap();
        println!("{}", character);
    }
}

fn read_file() -> Vec<String> {
    let file = BufReader::new(File::open("test_input.txt").unwrap());
    file.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| line.len() > 0)
        .collect()
}
