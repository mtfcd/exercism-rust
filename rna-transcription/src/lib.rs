use std::convert::{TryFrom, TryInto};

// fn make_xna(xna: &str, valid: [char; 4]) -> Result<Vec<char>, usize> {
//     let mut nucleotides = Vec::with_capacity(xna.len());
//     for (i, c) in xna.chars().enumerate() {
//         if !valid.contains(&c) {
//             return Err(i)
//         }
//         nucleotides.push(c);
//     }
//     Ok(nucleotides)
// }

macro_rules! xna_from_str {
    () => {
        pub fn new(dna: &str) -> Result<Self, usize> {
            let mut nucleotides = Vec::with_capacity(dna.len());
            for (i, c) in dna.chars().enumerate() {
                if let Ok(nuc) = c.try_into() {
                    nucleotides.push(nuc);
                } else {
                    return Err(i);
                }
            }
            Ok(Self { nucleotides })
        }
    };
}

#[derive(Debug, PartialEq)]
enum DnaNucleotide {
    A,
    C,
    T,
    G,
}
#[derive(Debug, PartialEq)]
enum RnaNucleotide {
    A,
    C,
    U,
    G,
}

impl TryFrom<char> for RnaNucleotide {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match &value {
            'G' => Ok(Self::G),
            'C' => Ok(Self::C),
            'A' => Ok(Self::A),
            'U' => Ok(Self::U),
            _ => Err(()),
        }
    }
}
impl TryFrom<char> for DnaNucleotide {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'G' => Ok(Self::G),
            'C' => Ok(Self::C),
            'A' => Ok(Self::A),
            'T' => Ok(Self::T),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Dna {
    nucleotides: Vec<DnaNucleotide>,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    nucleotides: Vec<RnaNucleotide>,
}

impl Dna {
    // pub fn new(dna: &str) -> Result<Dna, usize> {
    //     let nucleotides = make_xna(dna, ['A', 'C', 'T', 'G'])?;
    //     Ok(Self { nucleotides })
    // }
    xna_from_str!();

    pub fn into_rna(self) -> Rna {
        let nucleotides = self
            .nucleotides
            .into_iter()
            .map(|c| match c {
                DnaNucleotide::A => RnaNucleotide::U,
                DnaNucleotide::C => RnaNucleotide::G,
                DnaNucleotide::G => RnaNucleotide::C,
                DnaNucleotide::T => RnaNucleotide::A,
            })
            .collect();
        Rna { nucleotides }
    }
}

impl Rna {
    // pub fn new(rna: &str) -> Result<Rna, usize> {
    //     let nucleotides = make_xna(rna, ['A', 'C', 'U', 'G'])?;
    //     Ok(Self { nucleotides })
    // }
    xna_from_str!();
}
