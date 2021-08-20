use std::{collections::HashMap, iter::repeat};


const NUCLEOTIDES: [char; 4] = ['A', 'C', 'T', 'G'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }
    let mut count = 0;
    for e in dna.chars() {
        if !NUCLEOTIDES.contains(&e) {
            return Err(e);
        }
        if e == nucleotide {
            count += 1;
        }
    }
    return Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts: HashMap<char, usize> = NUCLEOTIDES.to_vec().into_iter().zip(repeat(0)).collect();
    for nu in dna.chars() {
        if !NUCLEOTIDES.contains(&nu) {
            return Err(nu)
        }
        match counts.get_mut(&nu) {
            Some(count) => *count += 1,
            None => {counts.insert(nu, 1);}
        }
    }
    Ok(counts)
}
