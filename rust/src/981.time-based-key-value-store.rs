use std::collections::HashMap;

struct TimeMap {
    store: HashMap<String, Vec<(String, i32)>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let entry = self.store.entry(key).or_insert(vec![]);
        entry.push((value, timestamp));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        use std::cmp::Ordering;
        let entry = self.store.get(&key);
        match entry {
            Some(v) => {
                let mut left = 0;
                let mut right = v.len() - 1;

                if timestamp < v[left].1 {
                    return "".to_string();
                }
                if timestamp >= v[right].1 {
                    return v[right].0.clone();
                }

                while left < right {
                    let mid = (left + right) / 2;

                    match &timestamp.cmp(&v[mid].1) {
                        Ordering::Equal => return v[mid].0.clone(),
                        Ordering::Less => right = mid,
                        Ordering::Greater => left = mid + 1,
                    }
                }
                v[left - 1].0.clone()
            }

            None => "".to_string(),
        }
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
