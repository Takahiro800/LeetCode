use crate::Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![i32::MAX; (amount + 1) as usize];
        dp[0] = 0;

        for &coin in &coins {
            for i in coin..=amount {
                dp[i as usize] = dp[i as usize].min(dp[(i - coin) as usize].saturating_add(1))
            }
        }

        match dp[amount as usize] {
            i32::MAX => -1,
            min_coins => min_coins,
        }
    }
}
