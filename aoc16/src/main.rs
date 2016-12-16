fn main() {
    let data = initial();
    let expanded = expand_to_size(data, 35651584);
    let check = checksum(expanded);
    print(&check);
}

fn initial() -> Vec<bool> {
    to_bit_vec("10001001100000001")
}

fn to_bit_vec(input: &str) -> Vec<bool> {
    input.chars().map(|c| c == '1').collect()
}

fn expand(a: Vec<bool>) -> Vec<bool> {
    let mut b = a.clone();
    b.reverse();
    b = b.iter().map(|x| !x).collect();

    let mut out = a.clone();
    out.push(false);
    out.append(&mut b);
    out
}

fn expand_to_size(init: Vec<bool>, size: usize) -> Vec<bool> {
    let mut out = init.clone();
    while out.len() < size {
        out = expand(out);
    }
    out.truncate(size);
    out
}

fn checksum(data: Vec<bool>) -> Vec<bool> {
    let mut check: Vec<bool> = data.chunks(2).map(|x| x[0] == x[1]).collect();
    if check.len() % 2 == 0 {
        check = checksum(check);
    }
    check
}

fn print(data: &Vec<bool>) {
    for &bit in data {
        print!("{}", if bit {'1'} else {'0'});
    }
    println!("");
}

#[test]
fn test_expand() {
    assert_eq!(expand(to_bit_vec("1")), to_bit_vec("100"));
    assert_eq!(expand(to_bit_vec("0")), to_bit_vec("001"));
    assert_eq!(expand(to_bit_vec("11111")), to_bit_vec("11111000000"));
    assert_eq!(expand(to_bit_vec("111100001010")), to_bit_vec("1111000010100101011110000"));
}

#[test]
fn test_expand_to_size() {
    assert_eq!(expand_to_size(to_bit_vec("10000"), 20), to_bit_vec("10000011110010000111"));
}

#[test]
fn test_checksum() {
    assert_eq!(checksum(to_bit_vec("10000011110010000111")), to_bit_vec("01100"));
}

