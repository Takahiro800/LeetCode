use crate::Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;

        let mut prev_row = vec![1_i32; n];
        let mut current_row = vec![1_i32; n];

        for _ in 1..m {
            for j in 1..n {
                current_row[j] = current_row[j - 1] + prev_row[j];
            }
            prev_row = current_row.clone();
        }

        *prev_row.last().unwrap()
    }
}
