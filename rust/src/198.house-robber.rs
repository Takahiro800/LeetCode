use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (a, b) = nums.iter().fold((0, 0), |(prev_a, prev_b), &num| {
            (prev_b + num, prev_a.max(prev_b))
        });

        a.max(b)
    }
}
