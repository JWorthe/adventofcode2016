use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;


struct Triangle {
    l1: i32,
    l2: i32,
    l3: i32
}

impl Triangle {
    fn is_valid(&self) -> bool {
        (self.l1 + self.l2) > self.l3 &&
            (self.l2 + self.l3) > self.l1 &&
            (self.l3 + self.l1) > self.l2
            
    }
}

fn main() {
    let len = read_file().iter().filter(|tri| tri.is_valid()).count();
    println!("{} valid triangles", len);
}

fn read_file() -> Vec<Triangle> {
    let file = BufReader::new(File::open("input.txt").unwrap());
    let mut lines = file.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| line.len() > 0)
        .peekable();

    let mut results = Vec::new();
    while lines.peek().is_some() {
        let (t11, t21, t31) = parse_line(lines.next().unwrap());
        let (t12, t22, t32) = parse_line(lines.next().unwrap());
        let (t13, t23, t33) = parse_line(lines.next().unwrap());

        results.push(Triangle {
            l1: t11,
            l2: t12,
            l3: t13
        });
        results.push(Triangle {
            l1: t21,
            l2: t22,
            l3: t23
        });
        results.push(Triangle {
            l1: t31,
            l2: t32,
            l3: t33
        });
    }
    results
}

fn parse_line(line: String) -> (i32, i32, i32) {
    let mut separated = line.split_whitespace();
    (separated.next().unwrap().trim().parse().unwrap(),
     separated.next().unwrap().trim().parse().unwrap(),
     separated.next().unwrap().trim().parse().unwrap())
}

