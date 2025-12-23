//go:build ignore

package main

func twoSum(v []int, sum int) []int {
	n := len(v)
	i, j := 0, n-1

	for i < j {
		if v[i]+v[j] == sum {
			break
		} else if v[i]+v[j] > sum {
			j--
		} else {
			i++
		}
	}

	return []int{i + 1, j + 1}
}

func main() {
	Assert(twoSum, []int{2, 7, 11, 15}, 9, []int{1, 2})
	Assert(twoSum, []int{1, 2, 3, 5, 9}, 8, []int{3, 4})
}
