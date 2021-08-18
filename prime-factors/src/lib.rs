// use std::iter::Iterator;

// pub struct PrimeGenerator {
//     primes: Vec<u64>,
// }

// impl PrimeGenerator {
//     pub fn new() -> Self {
//         Self { primes: vec![] }
//     }
// }

// impl Iterator for PrimeGenerator {
//     type Item = u64;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.primes.len() == 0 {
//             self.primes.push(2);
//             return Some(2);
//         }
//         let mut j = self.primes[self.primes.len() - 1];
//         if j == 2 {
//             self.primes.push(3);
//             return Some(3);
//         }
//         loop {
//             j += 2;
//             if !self.primes.iter().any(|i| j % i == 0) {
//                 break;
//             }
//         }
//         self.primes.push(j);
//         Some(j)
//     }
// }

// pub fn factors(n: u64) -> Vec<u64> {
//     let mut factors = Vec::new();
//     let mut cur_n = n;

//     let mut pg = PrimeGenerator::new();
//     while let Some(cur_prime) = pg.next() {
//         if cur_prime > cur_n {
//             break;
//         }
//         while cur_n % cur_prime == 0 {
//             cur_n /= cur_prime;
//             factors.push(cur_prime);
//         }
//     }

//     return factors;
// }

pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut can = 2..;
    let mut cur_n = n;

    while cur_n > 1 {
        let x = can.next().unwrap();

        while cur_n % x == 0 {
            cur_n /= x;
            result.push(x);
        }
    }

    return result;
}