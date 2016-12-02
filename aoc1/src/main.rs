use std::io::Read;
use std::fs::File;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }
    fn turn_left(self) -> Direction {
        //not an ambiturner
        self.turn_right().turn_right().turn_right()
    }

    fn as_vector(self, dist: i32) -> (i32,i32) {
        match self {
            Direction::Up => (0, -dist),
            Direction::Right => (dist, 0),
            Direction::Down => (0, dist),
            Direction::Left => (-dist, 0)
        }
    }
}

fn main() {
    let content = read_file().expect("Failed to read file");
    let (_, dist_x, dist_y) = first_repeated_dest(content);
    println!("Total: ({}, {})", dist_x, dist_y);
    println!("Net: {}", dist_x+dist_y);
}

fn net_distance(content: String) -> (Direction, i32, i32) {
    content.trim().split(", ")
        .map(|action|
             (
                 action.chars().nth(0).unwrap(),
                 action.chars().skip(1).collect::<String>().parse::<i32>().unwrap()
             ))
        .fold((Direction::Up, 0,0), |(facing, acc_x, acc_y), (dir, dist)| {
            let new_facing = match dir {
                'R' => facing.turn_right(),
                'L' => facing.turn_left(),
                _ => panic!("bad input")
            };
              
            let (new_x, new_y) = new_facing.as_vector(dist);
            (new_facing, acc_x+new_x, acc_y+new_y)
        })
}

fn first_repeated_dest(content: String) -> (Direction, i32, i32) {
    let mut stops: Vec<(i32, i32)> = Vec::new();
    let actions = content.trim().split(", ")
        .map(|action|
             (
                 action.chars().nth(0).unwrap(),
                 action.chars().skip(1).collect::<String>().parse::<i32>().unwrap()
             ));
    let mut current_facing = Direction::Up;
    let (mut acc_x, mut acc_y) = (0, 0);
    for (dir, dist) in actions {
        current_facing = match dir {
                'R' => current_facing.turn_right(),
                'L' => current_facing.turn_left(),
                _ => panic!("bad input")
            };

        for _ in 0..dist {
            let (new_x, new_y) = current_facing.as_vector(1);
            acc_x += new_x;
            acc_y += new_y;

            if stops.iter().any(|&(x, y)| x==acc_x && y==acc_y) {
                return (current_facing, acc_x, acc_y);
            }
            stops.push((acc_x, acc_y));
        }
    }
    (current_facing, acc_x, acc_y)
}

fn read_file() -> Result<String, String> {
    let mut file = try!(File::open("input.txt").map_err(|e| e.to_string()));
    let mut content = String::new();
    try!(file.read_to_string(&mut content).map_err(|e| e.to_string()));
    Ok(content)
}
