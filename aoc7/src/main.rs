use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let lines = read_file();
    let mut abba_count = 0;
    for line in lines {
        let mut in_squares = false;
        let mut char_buffer = [' '; 4];

        let mut chars = line.chars();
        char_buffer[0] = chars.next().unwrap();
        char_buffer[1] = chars.next().unwrap();
        char_buffer[2] = chars.next().unwrap();
        char_buffer[3] = chars.next().unwrap();
        let mut abba_found = is_abba(char_buffer);
        
        for c in chars {
            match c {
                '[' => {in_squares = true;},
                ']' => {in_squares = false;},
                _ => {}
            }
            char_buffer[0] = char_buffer[1];
            char_buffer[1] = char_buffer[2];
            char_buffer[2] = char_buffer[3];
            char_buffer[3] = c;

            if is_abba(char_buffer) {
                if in_squares {
                    abba_found = false;
                    break;
                } else {
                    abba_found = true;
                }
            }
        }

        if abba_found {
            abba_count += 1;
        }
    }
    println!("ABBA_count: {}", abba_count);
}

fn read_file() -> Vec<String> {
    let file = BufReader::new(File::open("input.txt").unwrap());
    file.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| line.len() > 0)
        .collect()
}

fn is_abba(char_buffer: [char; 4]) -> bool {
    char_buffer[0] != char_buffer[1] &&
    char_buffer[0] == char_buffer[3] &&
        char_buffer[1] == char_buffer[2] &&
        char_buffer[0] != ']' &&
        char_buffer[1] != ']' &&
        char_buffer[0] != '[' &&
        char_buffer[1] != '['
}
