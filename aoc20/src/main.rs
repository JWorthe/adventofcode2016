use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::u32;

#[derive(Debug)]
struct IpRange {
    start: u32,
    end: u32
}

impl IpRange {
    fn new(start: u32, end: u32) -> IpRange {
        IpRange {
            start: start,
            end: end
        }
    }
    
    fn contains(&self, other: u32) -> bool {
        self.start <= other && other <= self.end
    }

    fn try_combine(&self, other: &IpRange) -> Option<IpRange> {
        if self.contains(other.start) && self.contains(other.end) {
            Some(IpRange::new(self.start, self.end))
        }
        else if other.contains(self.start) && other.contains(self.end) {
            Some(IpRange::new(other.start, other.end))
        }
        else if self.contains(other.start) && other.contains(self.end) {
            Some(IpRange::new(self.start, other.end))
        }
        else if other.contains(self.start) && self.contains(other.end) {
            Some(IpRange::new(other.start, self.end))
        }
        else {
            None
        }
    }
}

fn main() {
    let mut ranges = read_file();
    optimize_ranges(&mut ranges);

    let mut allowed = Vec::new();

    // current will be in the u32 range while it's in the loop, but
    // needs to be a u64 to pass u32::MAX. Otherwise it will just
    // overflow and run forever.
    let mut current: u64 = 0;
    while current <= u32::MAX as u64 {
        match ranges.iter().find(|range| range.contains(current as u32)) {
            Some(blacklisting_range) => {
                current = blacklisting_range.end as u64 + 1;
            },
            None => {
                allowed.push(current as u32);
                current += 1;
            }
        }
    }
    
    let min_not_in_range = allowed[0];
    let allowed_count = allowed.len();

    println!("Min not in any range: {}", min_not_in_range);
    println!("Allowed count: {}", allowed_count);
}

fn read_file() -> Vec<IpRange> {
    let file = BufReader::new(File::open("input.txt").unwrap());
    file.lines()
        .filter_map(|line| {
            let line = line.unwrap();
            let mut split = line.split('-');
            let start = split.next();
            let end = split.next();
            match (start, end) {
                (Some(start), Some(end)) => Some(IpRange {
                    start: start.parse().unwrap(),
                    end: end.parse().unwrap()
                }),
                _ => None
            }
        })
        .collect()
}

fn optimize_ranges(ranges: &mut Vec<IpRange>) {
    let mut before_count = ranges.len();
    let mut after_count = 0;
    while before_count != after_count {
        before_count = ranges.len();
        ranges.sort_by_key(|r| r.start);
        
        let mut i = 0;
        while i < ranges.len()-1 {
            match ranges[i].try_combine(&ranges[i+1]) {
                Some(combined) => {
                    ranges[i] = combined;
                    ranges.remove(i+1);
                },
                None => {}
            }
            i += 1;
        }

        after_count = ranges.len();

        println!("Number of ranges {} => {}", before_count, after_count);
    }
}
