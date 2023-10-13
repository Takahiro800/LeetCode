impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;
        let mut appear = std::collections::HashSet::new();
        let mut window: Vec<char> = vec![];

        for char in s.chars() {
            while appear.contains(&char) {
                if let Some(c) = window.get(0) {
                    appear.remove(c);
                    window.remove(0);
                }
            }
            appear.insert(char);
            window.push(char);

            if window.len() > max {
                max = window.len();
            }
        }
        max as i32
    }
}
