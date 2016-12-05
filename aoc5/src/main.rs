extern crate md5;

fn main() {
    let room = "reyedfim";

    let mut i = 0;
    let mut found_bytes = 0;

    let mut code: [Option<u8>; 8] = [None;8];

    println!("Cracking the passwordz");
    print_code(&code);
    
    while found_bytes < 8 {
        let room_string = format!("{}{}", room, i);
        let room_bytes = room_string.into_bytes();
        let hash = md5::compute(room_bytes.as_slice());
        if match_hash(hash) {
            let position = hash[2];
            let value = hash[3] / 16;
            if code[position as usize].is_none() {
                code[position as usize] = Some(value);
                print_code(&code);
                found_bytes += 1;
            }
        }
        
        i+=1;
    }

    println!("Password found!");
}

fn match_hash(hash: [u8; 16]) -> bool {
    hash[0] == 0 &&
        hash[1] == 0 &&
        hash[2] < 8
}

fn print_code(code: &[Option<u8>; 8]) {
    println!("");
    for &byte in code.iter() {
        match byte {
            None => {print!("-");},
            Some(x) => {print!("{:x}", x);}
        }
    }
    println!("");
}
