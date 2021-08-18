pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // 1.
    // (1..limit).filter(|i| factors.iter().any(|j| *j != 0 && i % j == 0)).sum()

    // 2.
    let mut list = vec![false; limit as usize + 1];
    factors.iter()
        .filter(|i| **i != 0)
        .for_each(|i| {
            let mut j = 1;
            while j * i < limit {
                list[(j * i) as usize] = true;
                j += 1;
            }
        });
    
    list.iter().enumerate().filter(|(_, v)| **v).map(|(i, _)| i as u32).sum()
}
