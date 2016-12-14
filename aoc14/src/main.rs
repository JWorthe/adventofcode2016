extern crate md5;

use std::collections::HashMap;

fn main() {
//    let input = "abc";
    let input = "yjdafjpo";
    let mut index = 0;
    let mut results_found = 0;

    let mut hash_memo = HashMap::new();
    while results_found < 64 {
        let hash = stretched_hash(format!("{}{}", input, index), &mut hash_memo);
        
        let threes = find_concurrent_symbols(&hash, 3, true);
        if threes.len() > 0 {
//            println!("Found three at {} -> {}", index, hash);
            for i in 1..1001 {
                let hash = stretched_hash(format!("{}{}", input, index+i), &mut hash_memo);
                let fives = find_concurrent_symbols(&hash, 5, false);
                if fives.iter().any(|c| threes.contains(c)) {
                    results_found += 1;
//                    println!("Five found at {} -> {}", index+i, hash);
                    println!("Found hash {} at index {}", results_found, index);
                    break;
                }
            }
        }
        index += 1;
    }
    
}

fn find_concurrent_symbols(hash: &String, count: u8, exit_early: bool) -> Vec<char> {
    let mut last_symbol = None;
    let mut last_symbol_run = 0;
    let mut matches = Vec::new();
    for c in hash.chars() {
        let symbol_matches = match last_symbol {
            Some(s) => s == c,
            None => false
        };

        if symbol_matches {
            last_symbol_run += 1;
            if last_symbol_run >= count && !matches.contains(&c) {
                matches.push(c);
                if exit_early {
                    break;
                }
            }
        } else {
            last_symbol = Some(c);
            last_symbol_run = 1;
        }
    }

    matches
}

fn hash_to_string(hash: &[u8; 16]) -> String {
    let mut result = String::with_capacity(32);
    
    for &byte in hash.iter() {
        result.push_str(format!("{:02x}", byte).as_ref());
    }
    result
}

fn stretched_hash(input: String, memo: &mut HashMap<String, String>) -> String {
    memo.entry(input.clone()).or_insert_with(|| {
        let mut result = input;
        for _ in 0..2017 {
            result = string_hash(result);
        }
        result
    }).clone()
}

fn string_hash(input: String) -> String {
    let bytes_to_hash = input.into_bytes();
    let hash = md5::compute(bytes_to_hash.as_slice());
    hash_to_string(&hash)
}
