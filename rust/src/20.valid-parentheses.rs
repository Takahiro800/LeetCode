use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for char in s.chars() {
            match char {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' if Some(char) != stack.pop() => return false,
                _ => (),
            }
        }

        stack.is_empty()
    }
}
