package main

func findMin(nums []int) int {
	left := 0
	right := len(nums) - 1

	if nums[left] < nums[right] {
		return nums[left]
	}

	for left < right {
		mid := (left + right) / 2

		if nums[0] > nums[mid] {
			right = mid
		} else {
			left = mid + 1
		}
	}

	return nums[left]
}
