use crate::Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if target <= nums[0] {
            return 0;
        }

        let mut ok = 0_i32;
        let mut ng = nums.len() as i32;

        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            let num = nums[mid as usize];

            if num == target {
                return mid;
            } else if num < target {
                ok = mid;
            } else {
                ng = mid
            }
        }

        if nums[ok as usize] == target {
            ok
        } else {
            ng
        }
    }
}
