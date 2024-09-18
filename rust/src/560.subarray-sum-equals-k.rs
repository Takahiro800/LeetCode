use std::collections::HashMap;

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum_count = HashMap::new();
        sum_count.insert(0, 1);

        let (mut sum, mut ans) = (0, 0);

        for num in nums {
            sum += num;

            if let Some(&count) = sum_count.get(&(sum - k)) {
                ans += count;
            }

            *sum_count.entry(sum).or_insert(0) += 1;
        }

        ans
    }
}
