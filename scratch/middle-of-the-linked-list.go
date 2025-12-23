// https://leetcode.com/problems/middle-of-the-linked-list/

//go:build ignore

package main

import (
	"fmt"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func middleNode(head *ListNode) *ListNode {
	i, slow, fast := 1, head, head
	for fast != nil {
		if i%2 == 0 {
			slow = slow.Next
		}
		fast = fast.Next
		i += 1
	}
	return slow
}

func main() {
	n0, n1, n2, n3 := &ListNode{0, nil},
		&ListNode{1, nil},
		&ListNode{2, nil},
		&ListNode{3, nil}
	n0.Next = n1
	n1.Next = n2
	n2.Next = n3
	fmt.Println(middleNode(n0))
}
