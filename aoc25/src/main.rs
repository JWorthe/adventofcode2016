extern crate regex;
use regex::Regex;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

enum Instruction {
    Inc(usize),
    Dec(usize),

    TglLit(i32),
    TglReg(usize),

    OutLit(i32),
    OutReg(usize),
    
    CpyLitReg(i32, usize),
    CpyRegReg(usize, usize),
    CpyLitLit(i32, i32),
    CpyRegLit(usize, i32),
    
    JnzLitLit(i32, i32),
    JnzRegLit(usize, i32),
    JnzLitReg(i32, usize),
    JnzRegReg(usize, usize),

    Noop
}

impl Instruction {
    fn parse(line: &str) -> Instruction {
        match Regex::new(r"inc (a|b|c|d)").unwrap().captures(line) {
            Some(cap) => {
                let reg = to_register_index(cap.at(1).unwrap());
                return Instruction::Inc(reg);
            },
            _ => {}
        }

        match Regex::new(r"dec (a|b|c|d)").unwrap().captures(line) {
            Some(cap) => {
                let reg = to_register_index(cap.at(1).unwrap());
                return Instruction::Dec(reg);
            },
            _ => {}
        }
        

        match Regex::new(r"tgl (a|b|c|d)").unwrap().captures(line) {
            Some(cap) => {
                let reg = to_register_index(cap.at(1).unwrap());
                return Instruction::TglReg(reg);
            },
            _ => {}
        }

        match Regex::new(r"tgl ([-\d]+)").unwrap().captures(line) {
            Some(cap) => {
                let lit = cap.at(1).unwrap().parse().unwrap();
                return Instruction::TglLit(lit);
            },
            _ => {}
        }


        match Regex::new(r"out (a|b|c|d)").unwrap().captures(line) {
            Some(cap) => {
                let reg = to_register_index(cap.at(1).unwrap());
                return Instruction::OutReg(reg);
            },
            _ => {}
        }

        match Regex::new(r"out ([-\d]+)").unwrap().captures(line) {
            Some(cap) => {
                let lit = cap.at(1).unwrap().parse().unwrap();
                return Instruction::OutLit(lit);
            },
            _ => {}
        }
        
        
        match Regex::new(r"cpy ([-\d]+) (a|b|c|d)").unwrap().captures(line) {
            Some(cap) => {
                let src = cap.at(1).unwrap().parse().unwrap();
                let dest = to_register_index(cap.at(2).unwrap());
                return Instruction::CpyLitReg(src, dest);
            },
            _ => {}
        }

        match Regex::new(r"cpy (a|b|c|d) (a|b|c|d)").unwrap().captures(line) {
            Some(cap) => {
                let src = to_register_index(cap.at(1).unwrap());
                let dest = to_register_index(cap.at(2).unwrap());
                return Instruction::CpyRegReg(src, dest);
            },
            _ => {}
        }

        match Regex::new(r"cpy ([-\d]+) ([-\d]+)").unwrap().captures(line) {
            Some(cap) => {
                let src = cap.at(1).unwrap().parse().unwrap();
                let dest = cap.at(2).unwrap().parse().unwrap();
                return Instruction::CpyLitLit(src, dest);
            },
            _ => {}
        }

        match Regex::new(r"cpy (a|b|c|d) ([-\d]+)").unwrap().captures(line) {
            Some(cap) => {
                let src = to_register_index(cap.at(1).unwrap());
                let dest = cap.at(2).unwrap().parse().unwrap();
                return Instruction::CpyRegReg(src, dest);
            },
            _ => {}
        }

        
        match Regex::new(r"jnz ([-\d]+) (a|b|c|d)").unwrap().captures(line) {
            Some(cap) => {
                let src = cap.at(1).unwrap().parse().unwrap();
                let dest = to_register_index(cap.at(2).unwrap());
                return Instruction::JnzLitReg(src, dest);
            },
            _ => {}
        }

        match Regex::new(r"jnz (a|b|c|d) (a|b|c|d)").unwrap().captures(line) {
            Some(cap) => {
                let src = to_register_index(cap.at(1).unwrap());
                let dest = to_register_index(cap.at(2).unwrap());
                return Instruction::JnzRegReg(src, dest);
            },
            _ => {}
        }

        match Regex::new(r"jnz ([-\d]+) ([-\d]+)").unwrap().captures(line) {
            Some(cap) => {
                let src = cap.at(1).unwrap().parse().unwrap();
                let dest = cap.at(2).unwrap().parse().unwrap();
                return Instruction::JnzLitLit(src, dest);
            },
            _ => {}
        }

        match Regex::new(r"jnz (a|b|c|d) ([-\d]+)").unwrap().captures(line) {
            Some(cap) => {
                let src = to_register_index(cap.at(1).unwrap());
                let dest = cap.at(2).unwrap().parse().unwrap();
                return Instruction::JnzRegLit(src, dest);
            },
            _ => {}
        }
        
        panic!("Invalid instruction line");
    }

    fn toggle(&self) -> Self {
        match *self {
            Instruction::Inc(a) => Instruction::Dec(a),
            Instruction::Dec(a) => Instruction::Inc(a),
            
            Instruction::TglLit(_) => Instruction::Noop,
            Instruction::TglReg(a) => Instruction::Inc(a),

            Instruction::OutLit(_) => Instruction::Noop,
            Instruction::OutReg(a) => Instruction::Inc(a),
            
            Instruction::CpyLitReg(a, b) => Instruction::JnzLitReg(a, b),
            Instruction::CpyRegReg(a, b) => Instruction::JnzRegReg(a, b),
            Instruction::CpyLitLit(a, b) => Instruction::JnzLitLit(a, b),
            Instruction::CpyRegLit(a, b) => Instruction::JnzRegLit(a, b),
            
            Instruction::JnzLitLit(a, b) => Instruction::CpyLitLit(a, b),
            Instruction::JnzRegLit(a, b) => Instruction::CpyRegLit(a, b),
            Instruction::JnzLitReg(a, b) => Instruction::CpyLitReg(a, b),
            Instruction::JnzRegReg(a, b) => Instruction::CpyRegReg(a, b),
            Instruction::Noop => Instruction::Noop
        }
    }
}

fn main() {
    let expected_output = vec!(0, 1, 0, 1, 0, 1, 0, 1, 0, 1);
    let min_input = (0..).find(|&i| run_program(i) == expected_output).unwrap();
    println!("{}", min_input);
}


fn run_program(input: i32) -> Vec<i32> {
    let mut transmission = Vec::new();
    let mut program = read_file();

    let mut registers: [i32; 4] = [input, 0, 0, 0];
    let mut pc: usize = 0;
    
    while pc < program.len() && transmission.len() < 10 {
        let mut pc_next: usize = pc+1;

        match program[pc] {
            Instruction::Inc(dest) => {
                registers[dest] += 1;
            },
            Instruction::Dec(dest) => {
                registers[dest] -= 1;
            },
            
            Instruction::TglReg(offset) => {
                let line_to_toggle = (pc as i32 + registers[offset]) as usize;
                if line_to_toggle > 0 && line_to_toggle < program.len() {
                    program[line_to_toggle] = program[line_to_toggle].toggle();
                }
            },
            Instruction::TglLit(offset) => {
                let line_to_toggle = (pc as i32 + offset) as usize;
                if line_to_toggle > 0 && line_to_toggle < program.len() {
                    program[line_to_toggle] = program[line_to_toggle].toggle();
                }
            },

            Instruction::OutReg(output) => {
                transmission.push(registers[output]);
            },
            Instruction::OutLit(output) => {
                transmission.push(output);
            },
            
            Instruction::CpyLitReg(src, dest) => {
                registers[dest] = src;
            },
            Instruction::CpyRegReg(src, dest) => {
                registers[dest] = registers[src];
            },
            Instruction::CpyLitLit(_, _) | Instruction::CpyRegLit(_, _) => {},
            
            Instruction::JnzRegLit(condition, offset) => {
                if registers[condition] != 0 {
                    pc_next = (pc as i32 + offset) as usize;
                }
            },
            Instruction::JnzLitLit(condition, offset) => {
                if condition != 0 {
                    pc_next = (pc as i32 + offset) as usize;
                }
            },
            Instruction::JnzRegReg(condition, offset) => {
                if registers[condition] != 0 {
                    pc_next = (pc as i32 + registers[offset]) as usize;
                }
            },
            Instruction::JnzLitReg(condition, offset) => {
                if condition != 0 {
                    pc_next = (pc as i32 + registers[offset]) as usize;
                }
            },
            Instruction::Noop => {}
        }

        pc = pc_next;
    }
    
    transmission
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
