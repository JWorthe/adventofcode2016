extern crate regex;
use regex::Regex;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

enum Instruction {
    CpyLit(i32, usize),
    CpyReg(usize, usize),
    Inc(usize),
    Dec(usize),  
    Jnz(usize, i32),
    Jmp(i32),
    Noop
}

impl Instruction {
    fn parse(line: &str) -> Instruction {
        let cpy_lit: Regex = Regex::new(r"cpy ([-\d]+) (a|b|c|d)").unwrap();
        let cpy_reg = Regex::new(r"cpy (a|b|c|d) (a|b|c|d)").unwrap();
        let inc = Regex::new(r"inc (a|b|c|d)").unwrap();
        let dec = Regex::new(r"dec (a|b|c|d)").unwrap();
        let jnz_lit = Regex::new(r"jnz ([-\d]+) ([-\d]+)").unwrap();
        let jnz_reg = Regex::new(r"jnz (a|b|c|d) ([-\d]+)").unwrap();

        let cpy_lit_match = cpy_lit.captures(line);
        let cpy_reg_match = cpy_reg.captures(line);
        let inc_match = inc.captures(line);
        let dec_match = dec.captures(line);
        let jnz_lit_match = jnz_lit.captures(line);
        let jnz_reg_match = jnz_reg.captures(line);
        
        if cpy_lit_match.is_some() {
            let cap = cpy_lit_match.unwrap();
            let src: i32 = cap.at(1).unwrap().parse().unwrap();
            let dest = to_register_index(cap.at(2).unwrap());
            Instruction::CpyLit(src, dest)
        }
        else if cpy_reg_match.is_some() {
            let cap = cpy_reg_match.unwrap();
            let src = to_register_index(cap.at(1).unwrap());
            let dest = to_register_index(cap.at(2).unwrap());
            Instruction::CpyReg(src, dest)
        }
        else if inc_match.is_some() {
            let cap = inc_match.unwrap();
            let dest = to_register_index(cap.at(1).unwrap());
            Instruction::Inc(dest)
        }
        else if dec_match.is_some() {
            let cap = dec_match.unwrap();
            let dest = to_register_index(cap.at(1).unwrap());
            Instruction::Dec(dest)
        }
        else if jnz_lit_match.is_some() {
            let cap = jnz_lit_match.unwrap();
            let condition: i32 = cap.at(1).unwrap().parse().unwrap();
            let offset: i32 = cap.at(2).unwrap().parse().unwrap();
            if condition != 0 {
                Instruction::Jmp(offset)
            }
            else {
                Instruction::Noop
            }
        }
        else if jnz_reg_match.is_some() {
            let cap = jnz_reg_match.unwrap();
            let condition = to_register_index(cap.at(1).unwrap());
            let offset: i32 = cap.at(2).unwrap().parse().unwrap();
            Instruction::Jnz(condition, offset)
        }
        else {
            panic!("Invalid instruction line")
        }
    }
}

fn main() {
    let program = read_file();

    let mut registers: [i32; 4] = [0, 0, 1, 0];
    let mut pc: usize = 0;
    
    while pc < program.len() {
        let mut pc_next: usize = pc+1;

        match program[pc] {
            Instruction::CpyLit(src, dest) => {
                registers[dest] = src;
            },
            Instruction::CpyReg(src, dest) => {
                registers[dest] = registers[src];
            },
            Instruction::Inc(dest) => {
                registers[dest] += 1;
            },
            Instruction::Dec(dest) => {
                registers[dest] -= 1;
            },
            Instruction::Jnz(condition, offset) => {
                if registers[condition] != 0 {
                    pc_next = (pc as i32 + offset) as usize;
                }
            },
            Instruction::Jmp(offset) => {
                pc_next = (pc as i32 + offset) as usize
            },
            Instruction::Noop => {}
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

fn read_file() -> Vec<Instruction> {
    let file = BufReader::new(File::open("input.txt").unwrap());
    file.lines()
        .map(|line| line.unwrap())
        .filter(|line| line.len() > 0)
        .map(|line| Instruction::parse(line.trim()))
        .collect()
}
