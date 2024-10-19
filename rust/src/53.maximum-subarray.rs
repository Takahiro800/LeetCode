use crate::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current_sum = nums[0];
        let mut ans = nums[0];

        for &num in nums.iter().skip(1) {
            current_sum = num.max(current_sum + num);
            ans = ans.max(current_sum);
        }

        ans
    }
}
