extern crate md5;

#[derive(Clone)]
struct State {
    input: String,
    path: String,
    x: i8,
    y: i8
}

impl State {
    fn open_directions(&self) -> Vec<(char, i8, i8)> {
        let hash_input = format!("{}{}", self.input, self.path);
        let hash = md5::compute(hash_input.into_bytes().as_slice());

        let mut results = Vec::new();
        if hash[0]/16 > 10 && self.y > 0 {
            results.push(('U', 0, -1));
        }
        if hash[0]%16 > 10 && self.y < 3 {
            results.push(('D', 0, 1));
        }
        if hash[1]/16 > 10 && self.x > 0 {
            results.push(('L', -1, 0));
        }
        if hash[1]%16 > 10 && self.x < 3 {
            results.push(('R', 1, 0));
        }
        
        results
    }

    fn next_states(&self) -> Vec<State> {
        self.open_directions().iter()
            .map(|&(dir, dx, dy)| State {
                input: self.input.clone(),
                path: {
                    let mut p = self.path.clone();
                    p.push(dir);
                    p
                },
                x: self.x + dx,
                y: self.y + dy
            }).collect()
    }

    fn is_final(&self) -> bool {
        self.x == 3 && self.y == 3
    }
}

fn main() {
    let initial = State {
        input: "lpvhkcbi".to_string(),
        path: String::new(),
        x: 0,
        y: 0
    };

    let final_state = find_final_state(initial.clone());
    let longest_path = find_longest_path(initial);
    
    println!("Final State Path: {}", final_state.path);
    println!("Longest Path: {}", longest_path);
}

fn find_final_state(initial: State) -> State {
    let mut states = vec!(initial);
    
    loop {
        match states.iter().find(|s| s.is_final()) {
            Some(final_state) => {return final_state.clone();},
            None => {}
        };

        states = states.iter().flat_map(|s| s.next_states()).collect();
    }
}

fn find_longest_path(initial: State) -> u32 {
    let mut states = vec!(initial);
    let mut current_longest = 0;

    while states.len() > 0 {
        
        match states.iter().find(|s| s.is_final()) {
            Some(final_state) => {current_longest = final_state.path.len() as u32;},
            None => {}
        };

        states = states.iter()
            .filter(|s| !s.is_final())
            .flat_map(|s| s.next_states())
            .collect();
    }

    current_longest
}
