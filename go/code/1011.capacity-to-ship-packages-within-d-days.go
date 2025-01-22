package main

func shipWithinDays(weights []int, days int) int {
	canShip := func(cap int) bool {
		shipDays := 1
		curCap := cap

		for _, w := range weights {
			if curCap >= w {
				curCap -= w
			} else {
				shipDays++
				curCap = cap - w
			}
		}
		return shipDays <= days
	}

	max, sum := 0, 0
	for _, w := range weights {
		sum += w
		if w > max {
			max = w
		}
	}

	left, right := max, sum
	ans := sum
	for left <= right {
		mid := left + (right-left)/2

		if canShip(mid) {
			if mid < ans {
				ans = mid
			}
			right = mid - 1
		} else {
			left = mid + 1
		}
	}

	return ans
}
