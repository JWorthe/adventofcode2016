use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

struct Map {
    walls: Vec<Vec<bool>>,
    destinations: Vec<(usize, usize)>,
    start: (usize, usize)
}

impl Map {
    fn new(input: &[String]) -> Map {
        let mut walls = Vec::with_capacity(input.len());
        let mut start = (0, 0);
        let mut destinations = Vec::new();

        let mut y = 0;
        
        for line in input {
            let mut x = 0;
            let mut wall_row = Vec::with_capacity(line.len());
            for c in line.chars() {
                wall_row.push(c == '#');
                if c == '0' {
                    start = (y, x);
                }
                else if c.is_numeric() {
                    destinations.push((y, x));
                }
                
                x += 1;
            }

            walls.push(wall_row);
            y += 1;
        }

        Map {
            walls: walls,
            destinations: destinations,
            start: start
        }
    }

    fn is_valid_position(&self, y: i32, x: i32) -> bool {
        y > 0 &&
            x > 0 &&
            (y as usize) < self.walls.len() &&
            (x as usize) < self.walls[y as usize].len() &&
            !self.walls[y as usize][x as usize]
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct State {
    robot_location: (usize, usize),
    destinations_hit: Vec<bool>
}

impl State {
    fn new(map: &Map) -> State {
        State {
            robot_location: map.start,
            destinations_hit: map.destinations.iter().map(|_| false).collect()
        }
    }

    fn next_state(&self, movement: (i32, i32), map: &Map) -> Option<State> {
        let (y, x) = self.robot_location;
        let (dy, dx) = movement;
        let y2 = y as i32 + dy;
        let x2 = x as i32 + dx;

        if !map.is_valid_position(y2, x2) {
            return None;
        }

        let y2 = y2 as usize;
        let x2 = x2 as usize;

        Some(State {
            robot_location: (y2, x2),
            destinations_hit: self.destinations_hit.iter()
                .zip(map.destinations.iter())
                .map(|(&hit, &(dest_y, dest_x))| hit || (dest_y == y2 && dest_x == x2))
                .collect()
        })
    }
    
    fn next_states(&self, map: &Map) -> Vec<State> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)].iter()
            .filter_map(|&movement| self.next_state(movement, &map))
            .collect()
    }

    fn is_complete(&self, map: &Map) -> bool {
        self.destinations_hit.iter().all(|&x| x) && self.robot_location == map.start
    }
}

fn main() {
    let map = Map::new(&read_file());
    let initial = State::new(&map);

    let mut states = HashMap::new();
    states.insert(initial, 0);

    let mut moves = 0;
    
    while !states.iter().any(|(&ref s, _)| s.is_complete(&map)) {
        let new_states = states.iter()
            .filter(|&(_, &m)| m == moves)
            .flat_map(|(&ref s, _)| s.next_states(&map))
            .collect::<Vec<_>>();

        moves += 1;
        
        for state in new_states {
            if !states.contains_key(&state) {
                states.insert(state, moves);
            }
        }
    }

    println!("Final state found in {} moves", moves);
}

fn read_file() -> Vec<String> {
    let file = BufReader::new(File::open("input.txt").unwrap());
    file.lines()
        .map(|line| line.unwrap())
        .filter(|line| line.len() > 0)
        .collect()
}
