use crate::Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        if word_dict.is_empty() {
            return false;
        }

        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for i in 1..=s.len() {
            for word in &word_dict {
                let word_len = word.len();
                if word_len <= i && dp[i - word_len] && &s[i - word_len..i] == word {
                    dp[i] = true;
                    break;
                }
            }
        }
        *dp.last().unwrap_or(&false)
    }
}
