pub fn square(s: u32) -> u64 {
    1u64.wrapping_shl(s - 1)
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
