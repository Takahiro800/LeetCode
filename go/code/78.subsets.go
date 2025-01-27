package main

func subsets(nums []int) [][]int {
	n := len(nums)
	total := 1 << n
	ans := [][]int{}

	for mask := 0; mask < total; mask++ {
		subset := []int{}
		for i := 0; i < n; i++ {
			if mask&(1<<i) != 0 {
				subset = append(subset, nums[i])
			}
		}
		ans = append(ans, subset)
	}
	return ans
}
