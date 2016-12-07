use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let lines = read_file();
    let mut ssl_count = 0;
    for line in lines {
        let mut in_squares = false;
        let mut char_buffer = [' '; 3];

        let mut abas = Vec::new();
        let mut babs = Vec::new();

        let mut chars = line.chars();
        char_buffer[0] = chars.next().unwrap();
        char_buffer[1] = chars.next().unwrap();
        char_buffer[2] = chars.next().unwrap();
        if is_aba(char_buffer) {
            if in_squares {
                babs.push(char_buffer);
            } else {
                abas.push(char_buffer);
            }
        }
        
        for c in chars {
            match c {
                '[' => {in_squares = true;},
                ']' => {in_squares = false;},
                _ => {}
            }
            char_buffer[0] = char_buffer[1];
            char_buffer[1] = char_buffer[2];
            char_buffer[2] = c;

            if is_aba(char_buffer) {
                if in_squares {
                    babs.push(char_buffer);
                } else {
                    abas.push(char_buffer);
                }
            }
        }

        //println!("ABA {:?}", abas);
        //println!("BAB {:?}", babs);
        
        let mut is_ssl = false;
        for aba in &abas {
            for bab in &babs {
                is_ssl = is_ssl || correspond(aba.clone(), bab.clone());
            }
        }
        
        //println!("Is SSL? {}", is_ssl);
        
        if is_ssl {
            ssl_count += 1;
        }
    }
    println!("SSL_count: {}", ssl_count);
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

fn is_aba(char_buffer: [char; 3]) -> bool {
    char_buffer[0] != char_buffer[1] &&
    char_buffer[0] == char_buffer[2] &&
        char_buffer[0] != ']' &&
        char_buffer[1] != ']' &&
        char_buffer[0] != '[' &&
        char_buffer[1] != '['
}

fn correspond(aba: [char; 3], bab: [char; 3]) -> bool {
    //assumes both follow the aba pattern, so [0] == [2]
    aba[0] == bab[1] && aba[1] == bab[0]
}
