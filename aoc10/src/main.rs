extern crate regex;
use regex::Regex;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::cmp;

#[derive(Debug, Clone)]
struct Bot {
    low: Option<i32>,
    high: Option<i32>,
    low_dest: Option<Dest>,
    high_dest: Option<Dest>
}

#[derive(Debug, Clone)]
enum Dest {
    Bot(usize),
    Output(usize)
}

impl Bot {
    fn new() -> Bot {
        Bot {low: None, high: None, low_dest: None, high_dest: None}
    }
    
    fn add_input(&mut self, input: i32) {
        if self.low.is_none() {
            self.low = Some(input);
        }
        else {
            let other = self.low.unwrap(); //already handled none case
            self.low = Some(cmp::min(input,other));
            self.high = Some(cmp::max(input,other));
        }
    }
    fn ready(&self) -> bool {
        self.low.is_some() && self.high.is_some() &&
            self.low_dest.is_some() && self.high_dest.is_some()
    }
    fn clear(&mut self) {
        self.low = None;
        self.high = None;
    }
}


fn main() {
    let mut bots = build_bots_graph();
    let outputs = find_outputs(&mut bots);
    println!("Outputs {:?}", outputs);
}

fn find_outputs(bots: &mut Vec<Bot>) -> Vec<i32> {
    let mut output = Vec::new();
    
    let mut is_stable = false;
    while !is_stable {
        is_stable = true;
            
        for i in 0..bots.len() {
            if bots[i].ready() {
                is_stable = false;
                
                let low = bots[i].low.unwrap();
                let high = bots[i].high.unwrap();
                bots[i].clear();

                match bots[i].low_dest {
                    Some(Dest::Bot(j)) => {
                        bots[j].add_input(low);
                    },
                    Some(Dest::Output(j)) => {
                        check_add_output(&mut output, j);
                        output[j] = low;
                    },
                    _ => {}
                };
                match bots[i].high_dest {
                    Some(Dest::Bot(j)) => {
                        bots[j].add_input(high);
                    },
                    Some(Dest::Output(j)) => {
                        check_add_output(&mut output, j);
                        output[j] = high;
                    },
                    _ => {}
                };
            }
        }
    }

    output
}

fn build_bots_graph() -> Vec<Bot> {
    let lines = read_file();
    let mut bots = Vec::new();

    let value_regex = Regex::new(r"^value (\d+) goes to bot (\d+)$").unwrap();
    let give_regex = Regex::new(r"^bot (\d+) gives low to (output|bot) (\d+) and high to (output|bot) (\d+)$").unwrap();
    
    for line in lines {
        if value_regex.is_match(line.as_ref()) {
            let cap = value_regex.captures(line.as_ref()).unwrap();
            let value = cap.at(1).unwrap().parse().unwrap();
            let bot_index = cap.at(2).unwrap().parse().unwrap();
            check_add_bot(&mut bots, bot_index);
            bots[bot_index].add_input(value);
        }
        else if give_regex.is_match(line.as_ref()) {
            let cap = give_regex.captures(line.as_ref()).unwrap();
            let give_bot_index = cap.at(1).unwrap().parse().unwrap();
            let low_is_to_output = cap.at(2).unwrap() == "output";
            let low_dest = cap.at(3).unwrap().parse().unwrap();
            let high_is_to_output = cap.at(4).unwrap() == "output";
            let high_dest = cap.at(5).unwrap().parse().unwrap();

            check_add_bot(&mut bots, give_bot_index);
            bots[give_bot_index].low_dest = if low_is_to_output {
                Some(Dest::Output(low_dest))
            } else {
                Some(Dest::Bot(low_dest))
            };
            bots[give_bot_index].high_dest = if high_is_to_output {
                Some(Dest::Output(high_dest))
            } else {
                Some(Dest::Bot(high_dest))
            };
        }
    }

    bots
}

fn check_add_bot(bots: &mut Vec<Bot>, index: usize) {
    while index >= bots.len() {
        bots.push(Bot::new());
    }
}

fn check_add_output(outputs: &mut Vec<i32>, index: usize) {
    while index >= outputs.len() {
        outputs.push(0);
    }
}

fn read_file() -> Vec<String> {
    let file = BufReader::new(File::open("input.txt").unwrap());
    file.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| line.len() > 0)
        .collect()
}
