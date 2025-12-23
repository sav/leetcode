//go:build ignore

package main

import (
	"fmt"
	"math"
	"sort"
)

func threeSumClosest(v []int, target int) int {
	closest := math.MaxInt
	sum := math.MaxInt

	sort.Ints(v)

	for i, n := 0, len(v); i < n-2; i++ {
		j, k := i+1, n-1
		for j < k {
			sum := v[i] + v[j] + v[k]
			if sum == target {
				return sum
			} else if sum < target {
				j++
			} else {
				k--
			}
			curr := sum - target
			if curr < 0 { // Abs
				curr *= -1
			}
			if curr <= closest {
				closest = curr
				sum = sum
			}
		}
	}

	return sum
}

func assert(exp bool) {
	if !exp {
		panic("failed")
	}
}

func main() {
	assert(threeSumClosest([]int{-1, 2, 1, -4}, 1) == 2)
	assert(threeSumClosest([]int{0, 0, 0}, 0) == 0)
	fmt.Println("all tests passed.")
}
