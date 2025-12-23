// https://leetcode.com/problems/fruit-into-baskets/description/

//go:build ignore

package main

import (
	"fmt"
)

func contains(basket []int, fruit int) bool {
	return basket[0] == fruit || basket[1] == fruit
}

func replace(basket []int, last, fruit int) int {
	if basket[0] == last {
		basket[1] = fruit
		return basket[0]
	} else {
		basket[0] = fruit
		return basket[1]
	}
}

// Ideally, we shouldn't need this; just track counts as we go.
func prevCount(fruits []int, fruit, pos int) int {
	count := 0
	for i := pos; i >= 0; i-- {
		if fruits[i] == fruit {
			count += 1
		} else {
			break
		}
	}
	return count
}

func totalFruit(fruits []int) int {
	n := len(fruits)
	if n == 1 {
		return 1
	}

	basket := []int{fruits[0], fruits[1]}
	last := fruits[1]
	count, max := 2, 2

	for i := 2; i < n; i++ {
		if contains(basket, fruits[i]) {
			last = fruits[i]
			count += 1
		} else {
			prev := replace(basket, last, fruits[i])
			last = fruits[i]
			count = 1 + prevCount(fruits, prev, i-1)
		}
		if count > max {
			max = count
		}
	}
	return max
}

func main() {
	fmt.Println(totalFruit([]int{0, 1, 2, 2, 1, 0, 0, 0, 1, 1}))
	fmt.Println(totalFruit([]int{0, 0, 1, 1}))
}
