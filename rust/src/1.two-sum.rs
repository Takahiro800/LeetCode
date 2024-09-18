use std::collections::HashMap;

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let rem = target - num;

            match map.get(&rem) {
                Some(index) => return vec![*index, i as i32],
                None => map.insert(num, i as i32),
            };
        }

        vec![]
    }
}
