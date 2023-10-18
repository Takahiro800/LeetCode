impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut freq = [0; 26];
        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();

        for &byte in s1_bytes {
            freq[(byte - b'a') as usize] += 1;
        }

        let mut left = 0;
        let mut right = 0;

        while right < s2_bytes.len() {
            let idx = (s2_bytes[right] - b'a') as usize;

            if freq[idx] > 0 {
                freq[idx] -= 1;
                right += 1;
            } else if left == right {
                left += 1;
                right += 1;
            } else {
                freq[(s2_bytes[left] - b'a') as usize] += 1;
                left += 1;
            }

            if right - left == s1_bytes.len() {
                return true;
            }
        }

        false
    }
}
