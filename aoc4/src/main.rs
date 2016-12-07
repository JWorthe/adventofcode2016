use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;

#[derive(Debug)]
struct Room {
    encrypted_name: String,
    sector_id: u32,
    checksum: String
}

impl Room {
    fn new(line: String) -> Room {
        let name_length = line.find(char::is_numeric).unwrap();
        let (name, sector_and_check) = line.split_at(name_length);
        let (sector, check) = sector_and_check.split_at(sector_and_check.len()-7);
        
        Room {
            encrypted_name: name.trim_matches('-').to_string(),
            sector_id: sector.parse().unwrap(),
            checksum: check.trim_matches(|c| c=='[' || c==']').to_string()
        }
    }

    fn is_valid(&self) -> bool {
        let mut char_counts: BTreeMap<char, i32> = BTreeMap::new();
        let chars = self.encrypted_name.chars()
            .filter(|&c| c != '-');
        for c in chars {
            *char_counts.entry(c).or_insert(0) += 1;
        }

        let mut vec_char_counts = char_counts.iter().collect::<Vec<_>>();
        vec_char_counts.sort_by_key(|&(_, &value)| -value);
        let expected_checksum = vec_char_counts
            .iter()
            .take(5)
            .map(|&(&key, _)| key)
            .collect::<String>();

        expected_checksum == self.checksum
    }

    fn decrypt_name(&self) -> String {
        self.encrypted_name.chars().map(|c| {
            if c == '-' {
                ' '
            } else {
                let current_digit = c as u8 - 'a' as u8;
                let new_digit = ((current_digit as u32 + self.sector_id) % 26) as u8;
                (new_digit + 'a' as u8) as char
            }
        }).collect()
    }
}

fn main() {
    let rooms = read_file();
    let valid_rooms = rooms.iter()
        .filter(|r| r.is_valid())
        .collect::<Vec<_>>();
    let sector_sum: u32 = valid_rooms.iter()
        .map(|r| r.sector_id)
        .sum();

    println!("Sector sum {}", sector_sum);

    let north_poles = rooms.iter()
        .filter(|r| r.decrypt_name().contains("north"))
        .inspect(|r| println!("{} {}", r.decrypt_name(), r.sector_id))
        .collect::<Vec<_>>();
    println!("{:?}", north_poles);
}

fn read_file() -> Vec<Room> {
    let file = BufReader::new(File::open("input.txt").unwrap());
    file.lines()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| line.len() > 0)
        .map(Room::new)
        .collect()
}
