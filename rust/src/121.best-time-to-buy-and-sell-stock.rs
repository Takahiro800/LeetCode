use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold((i32::MAX, 0), |(buy_price, max_profit), &price| {
                (buy_price.min(price), max_profit.max(price - buy_price))
            })
            .1
    }
}
