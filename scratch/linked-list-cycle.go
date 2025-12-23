// https://leetcode.com/problems/linked-list-cycle/

//go:build ignore

package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

// Naive implementation uses a hashmap to track visited nodes.
// It may be faster in practice but uses O(n) space.

// Naive implementation. O(n) time, O(n) space.
func hasCycle_naive(p *ListNode) bool {
	m := make(map[*ListNode]bool)
	for p != nil {
		if m[p] {
			return true
		}
		m[p] = true
		p = p.Next
	}
	return false
}

// Fast/slow pointers approach (Floyd's algorithm). O(n) time, O(1) space.
// If a cycle exists, the fast pointer will eventually catch up to the slow one.
func hasCycle(head *ListNode) bool {
	if head == nil || head.Next == nil {
		return false
	}
	slow, fast := head, head.Next
	for fast != nil && fast.Next != nil {
		if slow == fast {
			return true
		}
		slow = slow.Next
		fast = fast.Next.Next
	}
	return false
}

func main() {
	n0, n1, n2, n3 := &ListNode{0, nil},
		&ListNode{1, nil},
		&ListNode{2, nil},
		&ListNode{3, nil}
	n0.Next = n1
	n1.Next = n2
	n2.Next = n3
	fmt.Println(hasCycle(n0))
	n3.Next = n0
	fmt.Println(hasCycle(n0))
}
