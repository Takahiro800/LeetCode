use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut hash = HashMap::new();
        let mut max = 0;
        let mut left: usize = 0;
        let mut right: usize = 0;

        fn frequent_count(hash: &HashMap<char, usize>) -> usize {
            let mut max = 0;
            for (_, value) in hash.iter() {
                max = std::cmp::max(max, *value);
            }
            max
        }

        for char in s.chars() {
            *hash.entry(char).or_insert(0) += 1;

            while right - left + 1 - frequent_count(&hash) > k as usize {
                *hash.get_mut(&s.chars().nth(left).unwrap()).unwrap() -= 1;
                left += 1;
            }

            max = std::cmp::max(right - left + 1, max);
            right += 1;
        }
        max as i32
    }
}
