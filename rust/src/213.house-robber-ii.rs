use crate::Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let length = nums.len();
        let first_value = nums[0];

        let (a, b) = nums.iter().enumerate().skip(1).fold(
            (first_value, first_value),
            |(prev_a, prev_b), (i, &num)| {
                if i == 1 || i == length - 1 {
                    (prev_a, prev_b)
                } else {
                    (prev_b + num, prev_a.max(prev_b))
                }
            },
        );

        let ans = a.max(b);

        let (a, b) = nums.iter().skip(1).fold((0, 0), |(prev_a, prev_b), &num| {
            (prev_b + num, prev_a.max(prev_b))
        });
        ans.max(a.max(b))
    }
}
