
use std::collections::HashMap;

//const MICROS: usize = 2; //example
const MICROS: usize = 7;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    microchips: [[bool; MICROS]; 4],
    generators: [[bool; MICROS]; 4],
    elevator: usize
}

impl State {
    fn is_final(&self) -> bool {
        let floor = 3;
        
        for i in 0..MICROS {
            if !self.microchips[floor][i] || !self.generators[floor][i] {
                return false;
            }
        }
        true
    }

    fn is_safe(&self) -> bool {
        for floor in 0..4 {
            for i in 0..MICROS {
                for j in 0..MICROS {
                    //need to be on same floor, if any other micro is there
                    if self.generators[floor][j] && !self.generators[floor][i] && self.microchips[floor][i] {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn valid_moves(&self) -> Vec<State> {
        let mut moves = Vec::new();

        let mut new_floors = Vec::new();
        if self.elevator > 0 {
            new_floors.push(self.elevator-1);
        }
        if self.elevator < 3 {
            new_floors.push(self.elevator+1);
        }
        for &new_floor in new_floors.iter() {

            //items to take can be: one micro, one generator, two micros, two generator, one of each

            //one micro
            for i in 0..MICROS {
                if self.microchips[self.elevator][i] {
                    moves.push(self.move_micro(i, self.elevator, new_floor));
                }
            }
            //one generator
            for i in 0..MICROS {
                if self.generators[self.elevator][i] {
                    moves.push(self.move_generator(i, self.elevator, new_floor));
                }
            }
            //two micros
            for i in 0..MICROS {
                for j in i+1..MICROS {
                    if self.microchips[self.elevator][i] && self.microchips[self.elevator][j] {
                        moves.push(self.move_micro(i, self.elevator, new_floor).move_micro(j, self.elevator, new_floor));
                    }
                }
            }
            //two generators
            for i in 0..MICROS {
                for j in i+1..MICROS {
                    if self.generators[self.elevator][i] && self.generators[self.elevator][j] {
                        moves.push(self.move_generator(i, self.elevator, new_floor).move_generator(j, self.elevator, new_floor));
                    }
                }
            }
            //one of each
            for i in 0..MICROS {
                for j in 0..MICROS {
                    if self.microchips[self.elevator][i] && self.generators[self.elevator][j] {
                        moves.push(self.move_micro(i, self.elevator, new_floor).move_generator(j, self.elevator, new_floor));
                    }
                }
            }
        }

        moves.iter().filter(|x| x.is_safe()).cloned().collect()
    }

    fn move_micro(&self, micro: usize, floor: usize, new_floor: usize) -> State {
        let mut new_state = self.clone();
        new_state.microchips[floor][micro] = false;
        new_state.microchips[new_floor][micro] = true;
        new_state.elevator = new_floor;
        new_state
    }
    fn move_generator(&self, gen: usize, floor: usize, new_floor: usize) -> State {
        let mut new_state = self.clone();
        new_state.generators[floor][gen] = false;
        new_state.generators[new_floor][gen] = true;
        new_state.elevator = new_floor;
        new_state
    }
}

fn main() {
    //Stronium, plutonium, thulium, ruthenium, curium, electrium, dilithium
    let initial = State {
        microchips:
        [[true, true, false, false, false, true, true],
         [false, false, false, true, true, false, false],
         [false, false, true, false, false, false, false],
         [false, false, false, false, false, false, false]],
        generators:
        [[true, true, false, false, false, true, true],
         [false, false, true, true, true, false, false],
         [false, false, false, false, false, false, false],
         [false, false, false, false, false, false, false]],
        elevator: 0
    };

    /*
    //example
    let initial = State {
        microchips:
        [[true, true],
         [false, false],
         [false, false],
         [false, false]],
        generators:
        [[false, false],
         [true, false],
         [false, true],
         [false, false]],
        elevator: 0
    };
    */
    
    let mut states: HashMap<State, u32> = HashMap::new();
    states.insert(initial, 0);

    let mut moves = 0;
    loop {
        if states.iter().any(|(state, _)| state.is_final()) {
            break;
        }

        let new_states: Vec<State> = states.iter().filter(|&(_, &x)| x == moves).flat_map(|(state, _)| state.valid_moves()).collect();

        moves += 1;

        for state in new_states {
            if !states.contains_key(&state) {
                states.insert(state, moves);
            }
        }
    }
    
    
    println!("Moves required: {}", moves);
}
