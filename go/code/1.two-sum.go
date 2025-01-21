package main

func twoSum(nums []int, target int) []int {
	numMap := make(map[int]int)

	for i, num := range nums {
		diff := target - num

		if idx, found := numMap[diff]; found {
			return []int{i, idx}
		}

		numMap[num] = i
	}

	return nil
}
