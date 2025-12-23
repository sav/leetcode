// https://leetcode.com/problems/sort-colors/
//
// The Dutch National Flag Problem is a computational problem proposed by Edsger Dijkstra.
//
// Entries from 0 up to (but not including) i are values less than MID,
//  entries from i up to (but not including) j are values equal to MID,
//  entries from j up to (and including) k are values not yet sorted, and
//  entries from k + 1 to the end of the array are values greater than MID.
//
// From: https://en.wikipedia.org/wiki/Dutch_national_flag_problem

//go:build ignore

package main

import "fmt"

const MID = 1 // middle value

func swap(a []int, i, j int) {
	if i != j {
		tmp := a[i]
		a[i] = a[j]
		a[j] = tmp
	}
}

func sortColors(a []int) {
	n := len(a)
	i, j, k := 0, 0, n-1

	for j <= k {
		if a[j] < MID {
			swap(a, i, j)
			i += 1
			j += 1
		} else if a[j] > MID {
			swap(a, j, k)
			k -= 1
		} else {
			j += 1
		}
	}
}

func main() {
	a := []int{1, 2, 1, 2, 0, 0, 1, 2, 1, 2}
	sortColors(a)
	fmt.Println(a)

	b := []int{1, 1, 1, 2, 2, 2, 0, 1, 2, 0}
	sortColors(b)
	fmt.Println(b)
}
