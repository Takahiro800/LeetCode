use std::collections::HashSet;

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut set: HashSet<String> = HashSet::new();

        for email in emails {
            let parts: Vec<&str> = email.split('@').collect();
            let local = parts[0].split('+').collect::<Vec<&str>>()[0];
            let local = local.replace(".", "");
            let email = local + "@" + parts[1];
            set.insert(email);
        }

        set.len() as i32
    }
}
