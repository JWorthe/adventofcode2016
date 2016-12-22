extern crate regex;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Node {
    x: i8,
    y: i8,
    size: i16,
    used: i16,
    avail: i16,
    blocker: bool
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reg = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+) +(\d+)T +(\d+)T +(\d+)T +(\d+)%").unwrap();
        let cap = match reg.captures(s) {
            Some(cap) => cap,
            None => return Err("Does not match regex".to_string())
        };
        Ok(Node {
            x: cap.at(1).unwrap().parse().unwrap(),
            y: cap.at(2).unwrap().parse().unwrap(),
            size: cap.at(3).unwrap().parse().unwrap(),
            used: cap.at(4).unwrap().parse().unwrap(),
            avail: cap.at(5).unwrap().parse().unwrap(),
            blocker: false
        })
    }
}

impl Node {
    fn is_viable_pair(&self, other: &Node) -> bool {
        (self.x != other.x || self.y != other.y) && self.used > 0 && self.used <= other.avail
    }

    fn recalculate_avail(&mut self) {
        self.avail = self.size - self.used;
        debug_assert!(self.avail >= 0);
        debug_assert!(self.used >= 0);
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Grid {
    nodes: Vec<Vec<Node>>,
    goal_x: i8,
    goal_y: i8
}

impl Grid {
    fn new(nodes: Vec<Node>) -> Grid {
        let mut grid_nodes = Vec::new();
        let mut next_col = Vec::new();

        let mut current_x = 0;
        
        for node in nodes {
            if current_x != node.x {
                grid_nodes.push(next_col);
                next_col = Vec::new();
                current_x += 1;
            }
            
            next_col.push(node);
        }
        grid_nodes.push(next_col);
        grid_nodes = Grid::normalize(grid_nodes);

        Grid {
            nodes: grid_nodes,
            goal_x: current_x,
            goal_y: 0
        }
    }

    fn normalize(mut nodes: Vec<Vec<Node>>) -> Vec<Vec<Node>> {
        for x in 0..nodes.len() {
            for y in 0..nodes[x].len() {
                nodes[x][y].size = ((nodes[x][y].size + 50)/100) * 100;
                nodes[x][y].used = ((nodes[x][y].used + 50)/100) * 100;
                nodes[x][y].avail = ((nodes[x][y].avail + 50)/100) * 100;
            }
        }
        nodes
    }

    fn is_final(&self) -> bool {
        self.goal_x == 0 && self.goal_y == 0
    }

    fn make_move(&self, x: i8, y: i8, dx: i8, dy: i8) -> Option<Grid> {
        if x+dx < 0 || x+dx >= self.nodes.len() as i8 || y+dy < 0 || y+dy >= self.nodes[x as usize].len() as i8 {
            return None;
        }
        if !self.nodes[x as usize][y as usize].is_viable_pair(&self.nodes[(x+dx) as usize][(y+dy) as usize]) {
            return None;
        }

        let mut new_grid = self.clone();
        new_grid.nodes[(x+dx) as usize][(y+dy) as usize].used += new_grid.nodes[x as usize][y as usize].used;
        new_grid.nodes[(x+dx) as usize][(y+dy) as usize].recalculate_avail();
        new_grid.nodes[x as usize][y as usize].used = 0;
        new_grid.nodes[x as usize][y as usize].recalculate_avail();
        
        if new_grid.goal_x == x && new_grid.goal_y == y {
            new_grid.goal_x = x+dx;
            new_grid.goal_y = y+dy;
        }

        Some(new_grid)
    }
    
    fn available_moves(&self) -> Vec<Grid> {
        let mut moves = Vec::with_capacity(4);
        
        for x in 0..self.nodes.len() as i8 {
            for y in 0..self.nodes[x as usize].len() as i8 {
                match self.make_move(x, y, -1, 0) {
                    Some(grid) => { moves.push(grid); },
                    None => {}
                };
                match self.make_move(x, y, 0, -1) {
                    Some(grid) => { moves.push(grid); },
                    None => {}
                };
                match self.make_move(x, y, 1, 0) {
                    Some(grid) => { moves.push(grid); },
                    None => {}
                };
                match self.make_move(x, y, 0, 1) {
                    Some(grid) => { moves.push(grid); },
                    None => {}
                };
            }
        }

        moves
    }
}


fn main() {
    let nodes = read_input();

    let initial = Grid::new(nodes);
    println!("Initial grid has {} possible moves", initial.available_moves().len());
    
    let mut explored: HashSet<Grid> = HashSet::new();
    let mut frontier: HashMap<Grid, u32> = HashMap::new();
    frontier.insert(initial, 0);
    let mut found_final = false;

    while !found_final {
        let (best_frontier, moves) = find_best_frontiers(&frontier);
        
        let new_states = best_frontier.available_moves();
        found_final = new_states.iter().any(|ref s| s.is_final());

        for state in new_states {
            if !(explored.contains(&state) || frontier.contains_key(&state)) {
                frontier.insert(state, moves+1);
            }
        }
        
        frontier.remove(&best_frontier);
        explored.insert(best_frontier);
    }

    let (final_frontier, moves) = frontier.iter().find(|&(s, _)| s.is_final()).unwrap();
    println!("It took {} moves to get the data", moves);
}

fn read_input() -> Vec<Node> {
    let file = BufReader::new(File::open("input.txt").unwrap());

    file.lines()
        .skip(2)
        .filter_map(|line| Node::from_str(line.unwrap().as_ref()).ok())
        .collect()
}

fn find_best_frontiers(frontier: &HashMap<Grid, u32>) -> (Grid, u32) {
    frontier.iter().min_by_key(|&(ref grid, &moves)| {
        grid.goal_x as u32 + grid.goal_y as u32 + moves
    }).map(|(&ref grid, &moves)| (grid.clone(), moves.clone())).unwrap()
}
