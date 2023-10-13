impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;
        let mut appear = std::collections::HashMap::new();

        let arr: Vec<char> = s.chars().collect();
        let mut left = 0;

        for right in 0..arr.len() {
            let char = arr[right];

            while appear.contains_key(&char) {
                appear.remove(&arr[left]);
                left += 1;
            }
            appear.insert(char, right);
            max = max.max(right - left + 1);
        }

        max as i32
    }
}
