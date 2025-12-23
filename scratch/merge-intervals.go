// https://leetcode.com/problems/merge-intervals

//go:build ignore

package main

import (
	"fmt"
	"slices"
)

func merge(intervals [][]int) [][]int {
	n := len(intervals)
	slices.SortFunc(intervals, func(a, b []int) int {
		return a[0] - b[0]
	})
	r := [][]int{intervals[0]}
	m := 0 // last index of r
	for i := 1; i < n; i++ {
		if intervals[i][0] <= r[m][1] { // overlap
			r[m][1] = max(intervals[i][1], r[m][1])
		} else {
			r = append(r, intervals[i])
			m++
		}
	}
	return r
}

func main() {
	fmt.Println(merge([][]int{{1, 3}, {2, 6}, {8, 10}, {15, 18}}))
	fmt.Println(merge([][]int{{1, 4}, {4, 5}}))
	fmt.Println(merge([][]int{{4, 7}, {1, 4}}))
	fmt.Println(merge([][]int{{2, 3}, {4, 5}, {6, 7}, {8, 9}, {1, 10}}))
	fmt.Println(merge([][]int{{1, 4}, {5, 6}}))
}
