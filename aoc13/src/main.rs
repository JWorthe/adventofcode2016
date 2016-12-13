extern crate regex;
use regex::Regex;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    //let target = (7, 4);
    let target = (31,39);

    let mut states: HashMap<(i32, i32), u32> = HashMap::new();
    states.insert((1, 1), 0);
    let mut moves = 0;
    while moves < 50 {
/*        if states.iter().any(|(&state, _)| state == target) {
            break;
        }
*/
        let new_states: Vec<(i32, i32)> = states.iter().filter(|&(_, &x)| x == moves).flat_map(|(state, _)| available_steps(state)).collect();

        moves += 1;

        for state in new_states {
            if !states.contains_key(&state) {
                states.insert(state, moves);
            }
        }
    }

    println!("Moves required: {}", moves);
    println!("Possible destinations: {}", states.iter().count()); 
}


fn available_steps(current: &(i32, i32)) -> Vec<(i32, i32)> {
    let &(x0, y0) = current;
    let mut results = Vec::new();
    for &(x, y) in [(x0-1, y0), (x0+1, y0), (x0, y0-1), (x0, y0+1)].iter() {
        if x >= 0 && y >= 0 && !is_wall(x, y) {
            results.push((x, y));
        }
    }
    results
}

fn print_map() {
    for y in 0..10 {
        for x in 0..10 {
            print!("{}", if is_wall(x, y) {'#'} else {'.'});
        }
        println!("");
    }
}

fn is_wall(x: i32, y: i32) -> bool {
//    let favourite_number = 10;
    let favourite_number = 1352;
    let funny_num = x*x + 3*x + 2*x*y + y + y*y + favourite_number;
    let bin = format!("{:b}", funny_num);
    let number_of_ones = bin.chars().filter(|&c| c == '1').count();
    number_of_ones % 2 == 1
}
