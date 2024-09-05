use std::collections::HashMap;

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for str in strs {
            let mut chars: Vec<char> = str.chars().collect();
            chars.sort_unstable();

            let key: String = chars.into_iter().collect();
            map.entry(key).or_insert(vec![]).push(str);
        }

        map.into_iter().map(|(_, value)| value).collect()
    }
}
