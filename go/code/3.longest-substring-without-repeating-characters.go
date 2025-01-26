package main

func lengthOfLongestSubstring(s string) int {
	max := 0
	appear := make(map[rune]bool)
	window := []rune{}

	for _, char := range s {
		for appear[char] {
			c := window[0]
			delete(appear, c)
			window = window[1:]
		}
		appear[char] = true
		window = append(window, char)

		if len(window) > max {
			max = len(window)
		}
	}
	return max
}
