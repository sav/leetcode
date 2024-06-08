package main

import (
	"fmt"
	"sort"
)

func threeSum(v []int) [][]int {
	r := make([][]int, 0)
	n := len(v)

	if n < 3 {
		return r
	}

	sort.Ints(v[:])

	m := make(map[string]bool)

	for i := 0; i < n-2; i++ {
		// optimize for equal elements
		if i > 0 && v[i] == v[i-1] {
			continue
		}

		j, k := i+1, n-1

		for j < k {
			c := v[i] + v[j] + v[k]

			if c == 0 {
				t := []int{v[i], v[j], v[k]}

				key := fmt.Sprintf("%v", t)
				if !m[key] {
					m[key] = true
					r = append(r, t)
				}

				// optimize for equal elements
				for j < k && v[j] == v[j]+1 {
					j += 1
				}
				for j < k && v[k] == v[k]-1 {
					k -= 1
				}

				j += 1
				k -= 1
			} else if c > 0 {
				k -= 1
			} else {
				j += 1
			}
		}
	}

	return r
}

// [2, 1, 1, 0, -2]

func main() {
	fmt.Println(threeSum([]int{-1, 0, 1, 2, -1, -4}))
	fmt.Println(threeSum([]int{0, 1, 1}))
	fmt.Println(threeSum([]int{0, 0, 0}))
	fmt.Println(threeSum([]int{1, -1, 0, 1}))
	fmt.Println(threeSum([]int{1, -1, 0, 1, -1, 2}))
	fmt.Println(threeSum([]int{-2, 0, 1, 1, 2}))
	fmt.Println(threeSum(make([]int, 3000)))
}
