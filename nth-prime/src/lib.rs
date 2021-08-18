pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2u32];
    if n == 0 {
        return primes[0];
    }
    for i in 3.. {
        if primes.iter().any(|p| i % p == 0) {
            continue;
        } else {
            primes.push(i)
        }
        if primes.len() > n as usize {
            break;
        }
    }
    return primes[n as usize];
}
