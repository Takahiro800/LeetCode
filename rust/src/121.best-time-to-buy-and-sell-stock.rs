impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold((i32::MAX, 0), |(min, max_profit), &el| {
                (el.min(min), max_profit.max(el - min))
            })
            .1
    }
}
