extern crate regex;
use regex::Regex;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;


fn main() {
    let program = read_file();

    let mut registers: [i32; 4] = [0;4];
    registers[2] = 1; //init c to 1
    let mut pc: usize = 0;

    let cpy_lit = Regex::new(r"cpy ([-\d]+) (a|b|c|d)").unwrap();
    let cpy_reg = Regex::new(r"cpy (a|b|c|d) (a|b|c|d)").unwrap();
    let inc = Regex::new(r"inc (a|b|c|d)").unwrap();
    let dec = Regex::new(r"dec (a|b|c|d)").unwrap();
    let jnz_lit = Regex::new(r"jnz ([-\d]+) ([-\d]+)").unwrap();
    let jnz_reg = Regex::new(r"jnz (a|b|c|d) ([-\d]+)").unwrap();

    
    while pc < program.len() {
        let line = program[pc].as_ref();
        let mut pc_next: usize = pc+1;
        
        if cpy_lit.is_match(line) {
            let cap = cpy_lit.captures(line).unwrap();
            let src: i32 = cap.at(1).unwrap().parse().unwrap();
            let dest = to_register_index(cap.at(2).unwrap());
            registers[dest] = src;
        }
        else if cpy_reg.is_match(line) {
            let cap = cpy_reg.captures(line).unwrap();
            let src = to_register_index(cap.at(1).unwrap());
            let dest = to_register_index(cap.at(2).unwrap());
            registers[dest] = registers[src];
        }
        else if inc.is_match(line) {
            let cap = inc.captures(line).unwrap();
            let dest = to_register_index(cap.at(1).unwrap());
            registers[dest] = registers[dest] + 1;
        }
        else if dec.is_match(line) {
            let cap = dec.captures(line).unwrap();
            let dest = to_register_index(cap.at(1).unwrap());
            registers[dest] = registers[dest] - 1;
        }
        else if jnz_lit.is_match(line) {
            let cap = jnz_lit.captures(line).unwrap();
            let condition: i32 = cap.at(1).unwrap().parse().unwrap();
            let offset: i32 = cap.at(2).unwrap().parse().unwrap();
            if condition != 0 {
                pc_next = (pc as i32 + offset) as usize;
            }
        }
        else if jnz_reg.is_match(line) {
            let cap = jnz_reg.captures(line).unwrap();
            let condition = to_register_index(cap.at(1).unwrap());
            let offset: i32 = cap.at(2).unwrap().parse().unwrap();
            if registers[condition] != 0 {
                pc_next = (pc as i32 + offset) as usize;
            }
        }

        pc = pc_next;
    }
    
    println!("a: {}, b: {}, c: {}, d: {}", registers[0], registers[1], registers[2], registers[3]);
    
}

fn to_register_index(name: &str) -> usize {
    match name {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => panic!("Invalid register provided")
    }
}

fn read_file() -> Vec<String> {
    let file = BufReader::new(File::open("input.txt").unwrap());
    file.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| line.len() > 0)
        .collect()
}
