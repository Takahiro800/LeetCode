impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        if nums[left] < nums[right] {
            return nums[left];
        }

        while left < right {
            let mid = (left + right) / 2;

            if nums[0] > nums[mid] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        nums[left]
    }
}
