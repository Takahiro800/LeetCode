use std::collections::HashMap;

struct LRUCache {
    capacity: i32,
    cache: HashMap<i32, i32>,
    keys: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::new(),
            keys: Vec::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(value) = self.cache.get(&key) {
            if let Some(index) = self.keys.iter().position(|&x| x == key) {
                self.keys.remove(index);
            }
            self.keys.push(key);
            return *value;
        }

        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(index) = self.keys.iter().position(|&x| x == key) {
            self.keys.remove(index);
        }

        if self.keys.len() >= self.capacity as usize {
            self.cache.remove(&self.keys[0]);
            self.keys.remove(0);
        }

        self.keys.push(key);
        if let Some(old_value) = self.cache.get_mut(&key) {
            *old_value = value;
        } else {
            self.cache.insert(key, value);
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
