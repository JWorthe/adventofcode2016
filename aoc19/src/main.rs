fn main() {
    let mut elves = (0..3017957).map(|i| i+1).collect::<Vec<_>>();

    let mut eliminate_mod = 1;
    
    while elves.len() > 1 {
        let next_mod = if elves.len() % 2 == eliminate_mod { 0 } else { 1 };
        elves = elves.iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 != eliminate_mod)
            .map(|(_, &e)| e)
            .collect();
        eliminate_mod = next_mod;
    }

    let elf = elves[0];
    println!("Elf {} gets all the presents", elf);
}
