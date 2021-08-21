pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let end = (upper_bound as f64).sqrt() as usize;
    let mut sieve = vec![true; upper_bound as usize + 1];
    for i in 2..=end {
        if !sieve[i] {
            continue;
        }
        sieve.iter_mut().skip(i * 2).step_by(i).map(|v| *v = false).last();
    }

    sieve.into_iter().enumerate().skip(2).filter(|(_, v)| *v).map(|(i, _)| i as u64).collect()
}
