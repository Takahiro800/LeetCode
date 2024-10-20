// use crate::Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        fn factorial(n: i32) -> i32 {
            (1..=n).product()
        }

        factorial(m + n - 2) / (factorial(m - 1) * factorial(n - 1))
    }
}
