package main

func lengthOfLongestSubstring(s string) int {
	start := 0
	ans := 0
	usedChar := make(map[byte]int)

	for i := 0; i < len(s); i++ {
		char := s[i]
		if _, ok := usedChar[char]; ok && usedChar[char] >= start {
			start = usedChar[char] + 1
		}

		ans = max(ans, i-start+1)
		usedChar[char] = i
	}
	return ans
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
