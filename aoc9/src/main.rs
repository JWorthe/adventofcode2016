use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, Clone, Copy)]
struct Marker {
    take: usize,
    repeat: usize
}
#[derive(Debug, Clone, Copy)]
enum Token {
    Marker(usize, Marker),
    Literal(usize)
}

impl Token {
    fn size(self) -> usize {
        match self {
            Token::Marker(x, _) => x,
            Token::Literal(x) => x
        }
    }

    fn is_marker(self) -> bool {
        match self {
            Token::Marker(_, _) => true,
            Token::Literal(_) => false
        }
    }

    fn unwrap_marker(self) -> Marker {
        match self {
            Token::Marker(_, marker) => marker,
            Token::Literal(_) => panic!("Bad unwrap marker")
        }
    }
}

fn main() {
    let mut tokens = parse(read_file());
    tokens.reverse();
    let mut length: u64 = 0;
    
    while tokens.len() > 0 {
        let next = tokens.pop().unwrap();
        
        if next.is_marker() {
            let marker = next.unwrap_marker();
            let mut used_take = 0;
            
            let mut tokens_to_repeat = Vec::new();
            while used_take < marker.take {
                let next_token = tokens.pop().unwrap();
                if used_take + next_token.size() <= marker.take {
                    tokens_to_repeat.push(next_token);
                }
                else {
                    assert!(!next_token.is_marker());
                    //assume splits only happen on literals
                    tokens_to_repeat.push(Token::Literal(marker.take - used_take));
                    tokens.push(Token::Literal(next_token.size() - marker.take + used_take));
                }
                used_take += next_token.size();
            }

            tokens_to_repeat.reverse();
            for _ in 0..marker.repeat {
                for &token in &tokens_to_repeat {
                    tokens.push(token);
                }
            }
        } else {
            length += next.size() as u64;
        }
    }
    println!("Length: {}", length);
}

fn parse(text: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = text.chars();
    let mut current_char = chars.next();
    while current_char.is_some() {
        let c = current_char.unwrap();
        if c == '(' {
            let mut char_count = 1;
            let mut take_str = String::new();
            let mut repeat_str = String::new();
            loop {
                let n = chars.next().unwrap();
                char_count += 1;
                if n == 'x' { break; }
                take_str.push(n);
            }
            loop {
                let n = chars.next().unwrap();
                char_count += 1;
                if n == ')' { break; }
                repeat_str.push(n);
            }

            let take: usize = take_str.parse().expect(format!("Tried to parse take {}", take_str).as_ref());
            let repeat: usize = repeat_str.parse().expect(format!("Tried to parse repeat {}", repeat_str).as_ref());
            tokens.push(Token::Marker(char_count , Marker{take: take, repeat:repeat}));
        } else {
            assert!(c != '(' && c != ')');
            tokens.push(Token::Literal(1));
        }
        
        current_char = chars.next();
    }
    collapse_literals(&mut tokens);
    
    tokens
}

fn collapse_literals(tokens: &mut Vec<Token>) {
    let mut i = 0;
    while i < tokens.len()-1 {
        match (tokens[i], tokens[i+1]) {
            (Token::Literal(l1), Token::Literal(l2)) => {
                tokens[i] = Token::Literal(l1+l2);
                tokens.remove(i+1);
            },
            _ => {
                i += 1;
            }
        }
    }
}

fn read_file() -> String {
    let file = BufReader::new(File::open("input.txt").unwrap());
    file.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| line.len() > 0)
        .next()
        .unwrap()
        .trim()
        .to_string()

//    "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN".to_string()
}
