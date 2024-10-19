use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = Vec::new();

        for &x in nums.iter() {
            match dp.binary_search(&x) {
                // Ok(pos) => dp[pos] = x,
                Ok(_) => (),
                Err(pos) => {
                    if pos == dp.len() {
                        dp.push(x);
                    } else {
                        dp[pos] = x;
                    }
                }
            }
        }

        dp.len() as i32
    }
}
