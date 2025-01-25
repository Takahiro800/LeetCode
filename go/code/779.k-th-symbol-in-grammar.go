package main

func kthGrammar(n int, k int) int {
	if n == 1 {
		return 0
	}

	prev := kthGrammar(n-1, (k+1)/2)
	if prev == 0 {
		return 1 - k%2
	}
	return k % 2
}
