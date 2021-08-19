pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::new();
        let pad = vec![0];
        for i in 0..row_count {
            if i == 0 {
                rows.push(vec![1]);
                continue;
            }
            if let Some(prev_row) = rows.get((i - 1) as usize) {
                let new_row: Vec<u32> = pad
                    .iter()
                    .chain(prev_row.iter())
                    .zip(prev_row.iter().chain(pad.iter()))
                    .map(|(a, b)| a + b)
                    .collect();
                rows.push(new_row)
            }
        }

        Self(rows)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}
