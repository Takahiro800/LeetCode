use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold((i32::MAX, 0), |(buy_price, profit), &price| {
                if price < buy_price {
                    (price, profit)
                } else {
                    (price, profit + (price - buy_price))
                }
            })
            .1
    }
}
