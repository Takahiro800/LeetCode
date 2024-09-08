use std::collections::HashSet;

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        emails
            .into_iter()
            .map(|email| Self::clean_email(email))
            .collect::<HashSet<_>>()
            .len() as i32
    }

    fn clean_email(email: String) -> String {
        let v: Vec<&str> = email.split('@').collect();

        let local = v[0]
            .chars()
            .take_while(|&char| char != '+')
            .filter(|&char| char != '.')
            .collect::<String>();

        [&local, v[1]].join("@")
    }
}
