use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let lines = read_file();
    let mut current = 5;
    for line in lines {
        current = line.chars().fold(current, |current, dir| move_char_hex(current, dir));
        println!("{:X}", current);
    }
}

fn read_file() -> Vec<String> {
    let file = BufReader::new(File::open("input.txt").unwrap());
    file.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| line.len() > 0)
        .collect()
}


//assume current in 1-9 range, and char is in UDLR
fn move_char(current: i32, dir: char) -> i32 {
    match dir {
        'U' => if current <= 3 { current } else { current - 3},
        'D' => if current >= 7 { current } else { current + 3},
        'L' => if current%3 == 1 { current } else { current - 1},
        'R' => if current%3 == 0 { current } else { current + 1},
        _ => panic!("Bad direction character")
    }
}

//     1
//   2 3 4
// 5 6 7 8 9
//   A B C
//     D
fn move_char_hex(current: i32, dir: char) -> i32 {
    match dir {
        'U' => match current {
            1|2|4|5|9 => current,
            3|13 => current - 2,
            _  => current - 4
        },
        'D' => match current {
            5|10|13|12|9 => current,
            1|11 => current + 1,
            _ => current + 4
        },
        'L' => match current {
            1|2|5|10|13 => current,
            _ => current - 1
        },
        'R' => match current {
            1|4|9|12|13 => current,
            _ => current + 1
        },
        _ => panic!("Bad direction character")
    }
}
