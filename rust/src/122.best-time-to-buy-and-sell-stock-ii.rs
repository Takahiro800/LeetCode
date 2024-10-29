use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold((i32::MAX, 0), |(buy_price, profit), &price| {
                (price, profit + 0.max(price - buy_price))
            })
            .1
    }
}
