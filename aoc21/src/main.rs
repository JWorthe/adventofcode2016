extern crate regex;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::VecDeque;

use regex::Regex;

fn main() {
    let password = "abcdefgh".to_string();
    let program = read_file();
    let scrambled = scramble_password(password, &program);
    println!("Scrambled password is {}", scrambled);
    let unscrambled = unscramble_password(scrambled, &program);
    println!("Which unscrambles to {}", unscrambled);

    let other_scrambled = "fbgdceah".to_string();
    let other_unscrambled = unscramble_password(other_scrambled, &program);
    
    println!("Decrypted easter bunny password is {}", other_unscrambled);
}

fn read_file() -> Vec<String> {
    let file = BufReader::new(File::open("input.txt").unwrap());
    file.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| line.len() > 0)
        .collect()
}

fn scramble_password(password: String, program: &Vec<String>) -> String {
    let mut pass = password.chars().collect::<VecDeque<_>>();    

    let instructions = scramble_instruction_set();
    
    for line in program {
        for &(ref reg, ref func) in &instructions {
            let captures = reg.captures(line.as_ref());
            if captures.is_some() {
                func(&mut pass, captures.unwrap());
            }
        }
    }

    pass.iter().cloned().collect()
}

fn unscramble_password(password: String, program: &Vec<String>) -> String {
    let mut pass = password.chars().collect::<VecDeque<_>>();

    let instructions = unscramble_instruction_set();

    let mut backwards_program = program.clone();
    backwards_program.reverse();
    for line in backwards_program {
        for &(ref reg, ref func) in &instructions {
            let captures = reg.captures(line.as_ref());
            if captures.is_some() {
                func(&mut pass, captures.unwrap());
            }
        }
    }

    pass.iter().cloned().collect()
}

fn scramble_instruction_set() -> Vec<(Regex, fn(&mut VecDeque<char>, regex::Captures))> {
    let mut instructions : Vec<(Regex, fn(&mut VecDeque<char>, regex::Captures))> = Vec::new();
    instructions.push((Regex::new(r"swap position (\d+) with position (\d+)").unwrap(), swap_pos));
    instructions.push((Regex::new(r"swap letter ([a-z]) with letter ([a-z])").unwrap(), swap_let));
    instructions.push((Regex::new(r"rotate left (\d+) step").unwrap(), rotate_left));
    instructions.push((Regex::new(r"rotate right (\d+) step").unwrap(), rotate_right));
    instructions.push((Regex::new(r"rotate based on position of letter ([a-z])").unwrap(), rotate_pos));
    instructions.push((Regex::new(r"reverse positions (\d+) through (\d+)").unwrap(), reverse_sub));
    instructions.push((Regex::new(r"move position (\d+) to position (\d+)").unwrap(), move_pos));
        
    instructions
}

fn unscramble_instruction_set() -> Vec<(Regex, fn(&mut VecDeque<char>, regex::Captures))> {
    let mut instructions : Vec<(Regex, fn(&mut VecDeque<char>, regex::Captures))> = Vec::new();
    instructions.push((Regex::new(r"swap position (\d+) with position (\d+)").unwrap(), swap_pos));
    instructions.push((Regex::new(r"swap letter ([a-z]) with letter ([a-z])").unwrap(), swap_let));
    instructions.push((Regex::new(r"rotate left (\d+) step").unwrap(), rotate_right));
    instructions.push((Regex::new(r"rotate right (\d+) step").unwrap(), rotate_left));
    instructions.push((Regex::new(r"rotate based on position of letter ([a-z])").unwrap(), undo_rotate_pos));
    instructions.push((Regex::new(r"reverse positions (\d+) through (\d+)").unwrap(), reverse_sub));
    instructions.push((Regex::new(r"move position (\d+) to position (\d+)").unwrap(), undo_move_pos));
        
    instructions
}

fn swap_pos(pass: &mut VecDeque<char>, cap: regex::Captures) {
    let src = cap.at(1).unwrap().parse::<usize>().unwrap();
    let dest = cap.at(2).unwrap().parse::<usize>().unwrap();
    pass.swap(src, dest);
}

fn swap_let(pass: &mut VecDeque<char>, cap: regex::Captures) {
    let src = cap.at(1).unwrap().chars().next().unwrap();
    let dest = cap.at(2).unwrap().chars().next().unwrap();
    let src_position = pass.iter().position(|&c| c == src);
    let dest_position = pass.iter().position(|&c| c == dest);
    match (src_position, dest_position) {
        (Some(src_pos), Some(dest_pos)) => {
            pass.swap(src_pos, dest_pos);
        },
        _ => {}
    };
}

fn rotate_left(pass: &mut VecDeque<char>, cap: regex::Captures) {
    let steps = cap.at(1).unwrap().parse::<usize>().unwrap();
    for _ in 0..steps {
        let c = pass.pop_front().unwrap();
        pass.push_back(c);
    }
}

fn rotate_right(pass: &mut VecDeque<char>, cap: regex::Captures) {
    let steps = cap.at(1).unwrap().parse::<usize>().unwrap();
    for _ in 0..steps {
        let c = pass.pop_back().unwrap();
        pass.push_front(c);
    }
}

fn rotate_pos(pass: &mut VecDeque<char>, cap: regex::Captures) {
    let letter = cap.at(1).unwrap().chars().next().unwrap();
    let pos = pass.iter().position(|&c| c == letter).unwrap();
    let steps = 1 + pos + if pos >=4 { 1 } else { 0 };
    for _ in 0..steps {
        let c = pass.pop_back().unwrap();
        pass.push_front(c);
    }
}

fn move_pos(pass: &mut VecDeque<char>, cap: regex::Captures) {
    let src = cap.at(1).unwrap().parse::<usize>().unwrap();
    let dest = cap.at(2).unwrap().parse::<usize>().unwrap();
    let c = pass.remove(src).unwrap();
    pass.insert(dest, c);
}

fn reverse_sub(pass: &mut VecDeque<char>, cap: regex::Captures) {
    let mut start = cap.at(1).unwrap().parse::<usize>().unwrap();
    let mut end = cap.at(2).unwrap().parse::<usize>().unwrap();
    while start < end {
        pass.swap(start, end);
        start += 1;
        end -= 1;
    }
}

fn undo_rotate_pos(pass: &mut VecDeque<char>, cap: regex::Captures) {
    let letter = cap.at(1).unwrap().chars().next().unwrap();
    let dest_pos = pass.iter().position(|&c| c == letter).unwrap();
    let mut pos = dest_pos;
        
    loop {
        let c = pass.pop_front().unwrap();
        pass.push_back(c);
        pos -= 1;
        let steps = 1 + pos + if pos >=4 { 1 } else { 0 };
        if (pos + steps) % pass.len() == dest_pos {
            break;
        }
    }
}

fn undo_move_pos(pass: &mut VecDeque<char>, cap: regex::Captures) {
    let src = cap.at(1).unwrap().parse::<usize>().unwrap();
    let dest = cap.at(2).unwrap().parse::<usize>().unwrap();
    let c = pass.remove(dest).unwrap();
    pass.insert(src, c);
}

#[test]
fn swap_pos_example() {
    let instructions = vec!("swap position 4 with position 0".to_string());
    let password = "abcde".to_string();
    assert_eq!("ebcda".to_string(), scramble_password(password, instructions));
}

#[test]
fn swap_let_example() {
    let instructions = vec!("swap letter d with letter b".to_string());
    let password = "ebcda".to_string();
    assert_eq!("edcba".to_string(), scramble_password(password, instructions));
}

#[test]
fn reverse_example() {
    let instructions = vec!("reverse positions 0 through 4".to_string());
    let password = "edcba".to_string();
    assert_eq!("abcde".to_string(), scramble_password(password, instructions));
}

#[test]
fn rotate_left_example() {
    let instructions = vec!("rotate left 1 step".to_string());
    let password = "abcde".to_string();
    assert_eq!("bcdea".to_string(), scramble_password(password, instructions));
}

#[test]
fn move_example() {
    let instructions = vec!("move position 3 to position 0".to_string());
    let password = "bdeac".to_string();
    assert_eq!("abdec".to_string(), scramble_password(password, instructions));
}

#[test]
fn rotate_pos_example_1() {
    let instructions = vec!("rotate based on position of letter b".to_string());
    let password = "abdec".to_string();
    assert_eq!("ecabd".to_string(), scramble_password(password, instructions));
}

#[test]
fn rotate_pos_example_2() {
    let instructions = vec!("rotate based on position of letter d".to_string());
    let password = "ecabd".to_string();
    assert_eq!("decab".to_string(), scramble_password(password, instructions));
}

#[test]
fn rotate_right_example() {
    let instructions = vec!("rotate right 2 steps".to_string());
    let password = "decab".to_string();
    assert_eq!("abdec".to_string(), scramble_password(password, instructions));
}
