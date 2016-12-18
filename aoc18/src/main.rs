fn main() {
    let init: Vec<char> = ".^^..^...^..^^.^^^.^^^.^^^^^^.^.^^^^.^^.^^^^^^.^...^......^...^^^..^^^.....^^^^^^^^^....^^...^^^^..^".chars().collect();
    
    let mut map = Vec::new();
    map.push(init);
    
    for _ in 1..400000 {
        let last = map.last().unwrap().clone();
        let mut next = Vec::new();
        for i in 0..last.len() {
            let left = if i == 0 { '.' } else { last[i-1] };
            let right = if i == last.len()-1 { '.' } else { last[i+1] };
            next.push(if left == right { '.' } else { '^' });
        }
        map.push(next);
    }

    let safe_count = map.iter().map(|row| row.iter().filter(|&&c| c=='.').count() as u32).sum::<u32>();
    println!("Safe tiles: {}", safe_count);
}
