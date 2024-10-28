use crate::Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let length = nums.len();

        if length == 1 {
            return nums[0];
        }

        fn rob_linear(nums: &[i32]) -> i32 {
            let (a, b) = nums.iter().fold((0, 0), |(prev_a, prev_b), &num| {
                (prev_b + num, prev_a.max(prev_b))
            });
            a.max(b)
        }

        let rob_first = rob_linear(&nums[..length - 1]);
        let not_rob_first = rob_linear(&nums[1..]);

        rob_first.max(not_rob_first)
    }
}
