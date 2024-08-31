use std::collections::HashMap;

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for &num in &nums {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut sorted = vec![Vec::new(); nums.len()];

        for (&num, &count) in map.iter() {
            sorted[nums.len() - count].push(num);
        }

        sorted.into_iter().flatten().take(k as usize).collect()
    }
}
