impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        use std::collections::HashMap;

        let mut hash = HashMap::new();
        let mut left: usize = 0;
        let mut right: usize = 0;

        for char in s1.chars() {
            *hash.entry(char).or_insert(0) += 1;
        }

        while right < s2.len() {
            if let Some(value) = hash.get_mut(&s2.chars().nth(right).unwrap()) {
                if *value == 0 {
                    while s2.chars().nth(left).unwrap() != s2.chars().nth(right).unwrap() {
                        *hash.get_mut(&s2.chars().nth(left).unwrap()).unwrap() += 1;
                        left += 1;
                    }
                    left += 1;
                } else {
                    *value -= 1;
                }
            } else {
                while left < right {
                    *hash.get_mut(&s2.chars().nth(left).unwrap()).unwrap() += 1;
                    left += 1;
                }
                left += 1;
            }
            right += 1;
            if right - left == s1.len() {
                return true;
            }
        }
        false
    }
}
