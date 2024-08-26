use std::collections::HashMap;
//
// use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let bracket_pairs: HashMap<char, char> = Self::get_barcket_pairs();
        let mut stack: Vec<char> = Vec::new();

        for char in s.chars() {
            if bracket_pairs.contains_key(&char) {
                stack.push(char);
            } else if Some(&char) != stack.pop().and_then(|c| bracket_pairs.get(&c)) {
                return false;
            }
        }

        stack.is_empty()
    }

    fn get_barcket_pairs() -> HashMap<char, char> {
        let mut map = HashMap::new();
        map.insert('(', ')');
        map.insert('[', ']');
        map.insert('{', '}');
        map
    }
}
