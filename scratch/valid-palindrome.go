//go:build ignore

package main

import (
	"regexp"
	"strings"
)

func isPalindrome(s string) bool {
	s = strings.ToLower(s)
	s = regexp.MustCompile(`[^0-9A-Za-z]`).ReplaceAllString(s, "")

	n := len(s)

	for i := 0; i < n/2; i++ {
		if s[i] != s[n-i-1] {
			return false
		}
	}

	return true
}

func main() {
	Assert(isPalindrome, "#ab1!c1B!A@", true)
	Assert(isPalindrome, "abba", true)
	Assert(isPalindrome, "abcba", true)
	Assert(isPalindrome, "ab_a", true)
	Assert(isPalindrome, "race a car", false)
}
