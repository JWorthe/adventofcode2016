use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let lines = read_file();
    let mut current = '5';
    for line in lines {
        current = line.chars().fold(current, |current, dir| move_char_2(current, dir));
        println!("{}", current);
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
fn move_char_2(current: char, dir: char) -> char {
    match dir {
        'U' => match current {
            '1'|'2'|'4'|'5'|'9' => current,
            '3' => '1',
            '6' => '2',
            '7' => '3',
            '8' => '4',
            'A' => '6',
            'B' => '7',
            'C' => '8',
            'D' => 'B',
            _ => panic!("Bad current char")
        },
        'D' => match current {
            'A'|'D'|'C'|'5'|'9' => current,
            '1' => '3',
            '2' => '6',
            '3' => '7',
            '4' => '8',
            '6' => 'A',
            '7' => 'B',
            '8' => 'C',
            'B' => 'D',
            _ => panic!("Bad current char")
        },
        'L' => match current {
            '1'|'2'|'5'|'A'|'D' => current,
            '3' => '2',
            '4' => '3',
            '6' => '5',
            '7' => '6',
            '8' => '7',
            '9' => '8',
            'B' => 'A',
            'C' => 'B',
            _ => panic!("Bad current char")
        },
        'R' => match current {
            '1'|'4'|'9'|'C'|'D' => current,
            '2' => '3',
            '3' => '4',
            '5' => '6',
            '6' => '7',
            '7' => '8',
            '8' => '9',
            'A' => 'B',
            'B' => 'C',
            _ => panic!("Bad current char")
        },
        _ => panic!("Bad direction character")
    }
}
