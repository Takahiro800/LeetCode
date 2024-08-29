use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    max_size: i32,
    heap: BinaryHeap<Reverse<i32>>,
}

#[allow(dead_code)]
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut _self = KthLargest {
            max_size: k,
            heap: BinaryHeap::with_capacity(k as usize + 1),
        };

        for num in nums {
            _self.add(num);
        }

        _self
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));

        if self.heap.len() > self.max_size as usize {
            self.heap.pop();
        }

        self.heap.peek().map_or(0, |&Reverse(val)| val)
    }
}
