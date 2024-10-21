// use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn first_uniq_char(s: String) -> i32 {
        let mut counts = [0; 26];

        for c in s.chars() {
            counts[(c as u8 - 'a' as u8) as usize] += 1;
        }

        for (i, c) in s.chars().enumerate() {
            if counts[(c as u8 - 'a' as u8) as usize] == 1 {
                return i as i32;
            }
        }
        -1
    }
}
