use std::{collections::BinaryHeap};
#[derive(Debug)]
pub struct HighScores<'a>(&'a [u32]);

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self(scores)
    }

    pub fn scores(&self) -> &[u32] {
        self.0
    }

    pub fn latest(&self) -> Option<u32> {
        match self.0.last() {
            Some(i) => Some(*i),
            None => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.0.iter().max() {
            Some(i) => Some(*i),
            None => None,
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut bh: BinaryHeap<i64> = BinaryHeap::with_capacity(3);
        self.0.iter().for_each(|i| {
            let i = *i as i64;
            if bh.len() < 3 {
                bh.push(-i);
                return;
            }
            if let Some(mut score) = bh.peek_mut() {
                if -(*score) < i {
                    *score = -i
                }
            };
        });

        let mut res = bh.into_iter().map(|i| -i as u32).collect::<Vec<u32>>();
        res.sort_by(|a, b| b.cmp(a));
        res
    }
}
