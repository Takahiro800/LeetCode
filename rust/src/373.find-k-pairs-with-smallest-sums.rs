use std::{cmp::Reverse, collections::BinaryHeap};

use crate::Solution;

#[allow(dead_code)]
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        let mut k = k as usize;
        let (m, n) = (nums1.len(), nums2.len());
        let mut ans = vec![];

        for i in 0..m {
            if heap.len() >= k {
                break;
            }

            heap.push(Reverse((nums1[i] + nums2[0], i, 0 as usize)));
        }

        while k > 0 {
            if let Some(Reverse((_sum, i, j))) = heap.pop() {
                ans.push(vec![nums1[i], nums2[j]]);

                if j + 1 < n {
                    heap.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
                }
            }
            k -= 1;
        }
        ans
    }
}
