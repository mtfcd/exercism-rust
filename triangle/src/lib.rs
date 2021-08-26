use std::{cmp::Ordering, collections::HashSet, fmt::Display, hash::Hash, ops::Add};

pub struct Triangle<T>([T; 3]);

pub trait Trigonometry<T> {
    fn build(sides: [T; 3]) -> Option<Self>
        where Self: Sized;

    fn is_equilateral(&self) -> bool;

    fn is_scalene(&self) -> bool;

    fn is_isosceles(&self) -> bool;
}

// this just for u64
impl Trigonometry<u64> for Triangle<u64> {
    fn build(sides: [u64; 3]) -> Option<Self> {
        if sides.iter().any(|v| *v == 0) {
            return None
        }
        let sum = sides.iter().sum::<u64>();
        let max = sides.iter().max().unwrap();
        if max + max > sum {
            return None
        }
        Some(Self(sides))
    }

    fn is_equilateral(&self) -> bool {
        self.0[0] == self.0[1] && self.0[1] == self.0[2]
    }

    fn is_scalene(&self) -> bool {
        self.0.iter().collect::<HashSet<_>>().len() == 3
    }

    fn is_isosceles(&self) -> bool {
        self.0.iter().collect::<HashSet<_>>().len() == 2
    }
}

#[derive(Clone, Copy)]
pub struct Side<T>(T);

impl<T: Display> Hash for Side<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{}", self.0).hash(state)
    }
}

impl<T: PartialEq> PartialEq for Side<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: PartialEq> Eq for Side<T> {}

impl<T: PartialOrd> PartialOrd for Side<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.0 == other.0 {
            Some(Ordering::Equal)
        } else if self.0 > other.0 {
            Some(Ordering::Greater)
        } else if self.0 < other.0 {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

impl<T: PartialOrd> Ord for Side<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            Ordering::Equal
        } else if self > other {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl<T: Add<Output = T>> Add for Side<T> {
    type Output = Side<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: PartialEq + PartialOrd + Default + Copy + Add<Output = T> + Display> Triangle<Side<T>> {
    pub fn build(sides: [T; 3]) -> Option<Self> {
        if sides.iter().any(|v| *v == T::default()) {
            return None
        }
        let sides: [Side<T>; 3] = [Side(sides[0]), Side(sides[1]), Side(sides[2])];
        let sum: Side<T> = sides[0] + sides[1] + sides[2];
        let max = sides.iter().max().unwrap();
        if *max + *max > sum {
            return None
        }
        Some(Self(sides))
    }
    pub fn is_equilateral(&self) -> bool {
        self.0[0] == self.0[1] && self.0[1] == self.0[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.0.iter().collect::<HashSet<_>>().len() == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.0.iter().collect::<HashSet<_>>().len() == 2
    }
}