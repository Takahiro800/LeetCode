package main

func combinationSum(candidates []int, target int) [][]int {
	ans := [][]int{}
	var backtrack func(target int, start int, combination []int)

	backtrack = func(target int, start int, combination []int) {
		if target == 0 {
			temp := make([]int, len(combination))
			copy(temp, combination)
			ans = append(ans, temp)
			return
		}

		for i := start; i < len(candidates); i++ {
			if target < candidates[i] {
				continue
			}

			combination = append(combination, candidates[i])
			backtrack(target-candidates[i], i, combination)
			combination = combination[:len(combination)-1]
		}
	}

	backtrack(target, 0, []int{})
	return ans
}
