fn main() {
    part1();
    part2();
}

fn part1() {
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
    println!("Elf {} gets all the presents in part 1", elf);
}

fn part2() {
    let mut elves = (0..3017957).map(|i| i+1).collect::<Vec<_>>();

    let mut i = 0;
    while elves.len() > 1 {
        i = i % elves.len();
        let to_eliminate = (i + elves.len() / 2) % elves.len();
//        println!("Elf {} is taking presents from elf {}", elves[i], elves[to_eliminate]);
        elves.remove(to_eliminate);
        if to_eliminate > i {
            i += 1;
        }
    }

    let elf = elves[0];
    println!("Elf {} gets all the presents in part 2", elf);

}
