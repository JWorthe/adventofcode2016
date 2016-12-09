extern crate regex;
use regex::Regex;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;


struct Screen {
    data: [[bool; 6]; 50]
}

impl Screen {
    fn new() -> Screen {
        Screen {
            data: [[false; 6]; 50]
        }
    }
    
    fn rect(&mut self, a: usize, b: usize) {
        for x in 0..a {
            for y in 0..b {
                self.data[x][y] = true;
            }
        }
    }

    fn rotate_row(&mut self, y: usize, b: usize) {
        let mut row_copy = [false; 50];
        for x in 0..50 {
            row_copy[x] = self.data[x][y];
        }
        for x in 0..50 {
            self.data[x][y] = row_copy[(x+50-b)%50];
        }
    }
    
    fn rotate_col(&mut self, x: usize, b: usize) {
        let mut col_copy = [false; 6];
        for y in 0..6 {
            col_copy[y] = self.data[x][y];
        }
        for y in 0..6 {
            self.data[x][y] = col_copy[(y+6-b)%6];
        }
    }

    fn print(&self) {
        for y in 0..6 {
            for x in 0..50 {
                print!("{}", if self.data[x][y] {'#'} else {'.'});
            }
            println!("");
        }
    }

    fn count_on(&self) -> u32 {
        let mut count = 0;
        for y in 0..6 {
            for x in 0..50 {
                if self.data[x][y] {
                    count += 1;
                }
            }
        }
        count
    }
}


fn main() {
    let lines = read_file();
    let mut screen = Screen::new();
    for line in lines {
        if line.starts_with("rect") {
            //rect 3x4
            let re = Regex::new(r"rect (\d+)x(\d+)").unwrap();
            let cap = re.captures(line.as_ref()).unwrap();
            println!("{} parsed as RECT. A={}, B={}",
                     line,
                     cap.at(1).unwrap_or(""), cap.at(2).unwrap_or(""));
            screen.rect(cap.at(1).unwrap().parse().unwrap(), cap.at(2).unwrap().parse().unwrap());
            
        }
        else if line.starts_with("rotate row") {
            //rotate row y=0 by 4
            let re = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
            let cap = re.captures(line.as_ref()).unwrap();
            println!("{} parsed as ROTATE ROW. A={}, B={}",
                     line,
                     cap.at(1).unwrap_or(""), cap.at(2).unwrap_or(""));
            screen.rotate_row(cap.at(1).unwrap().parse().unwrap(), cap.at(2).unwrap().parse().unwrap());
        }
        else if line.starts_with("rotate column") {
            //rotate column x=1 by 1
            let re = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
            let cap = re.captures(line.as_ref()).unwrap();
            println!("{} parsed as ROTATE COLUMN. A={}, B={}",
                     line,
                     cap.at(1).unwrap_or(""), cap.at(2).unwrap_or(""));
            screen.rotate_col(cap.at(1).unwrap().parse().unwrap(), cap.at(2).unwrap().parse().unwrap());
            
        }

        screen.print();
    }

    println!("On: {}", screen.count_on());
}

fn read_file() -> Vec<String> {
    let file = BufReader::new(File::open("input.txt").unwrap());
    file.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| line.len() > 0)
        .collect()
}
