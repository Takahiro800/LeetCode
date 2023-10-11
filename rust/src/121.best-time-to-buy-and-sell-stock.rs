impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 1;

        let mut current_max = 0;

        while right < prices.len() {
            if prices[left] < prices[right] {
                let profit = prices[right] - prices[left];
                if profit > current_max {
                    current_max = profit;
                }
            } else {
                left = right;
            }
            right += 1;
        }

        current_max
    }
}
