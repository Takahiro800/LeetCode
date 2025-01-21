package main

func search(nums []int, target int) int {
	left := 0
	right := len(nums) - 1

	if left == right {
		if nums[left] == target {
			return 0
		} else {
			return -1
		}
	}

	for left <= right {
		mid := left + (right-left)/2

		if nums[mid] == target {
			return mid
		}

		// border is nothing
		if nums[left] < nums[mid] && nums[mid] < nums[right] {
			if target < nums[mid] {
				right = mid
			} else {
				left = mid + 1
			}
		} else {
			// border is on the right
			if nums[left] <= nums[mid] {
				if nums[left] <= target && target < nums[mid] {
					right = mid
				} else {
					left = mid + 1
				}
			} else {
				// border is on the left
				if nums[mid] < target && target <= nums[right] {
					left = mid + 1
				} else {
					right = mid
				}
			}
		}
	}

	return -1
}
