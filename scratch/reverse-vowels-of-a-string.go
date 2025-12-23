// https://leetcode.com/problems/reverse-vowels-of-a-string

//go:build ignore

package main

import (
	"fmt"
)

func isVowel(r rune) bool {
	return r == 'A' || r == 'E' || r == 'I' || r == 'O' || r == 'U' ||
		r == 'a' || r == 'e' || r == 'i' || r == 'o' || r == 'u'
}

func reverseVowels(str string) string {
	s := []rune(str)
	v := []rune{}
	for _, r := range s {
		if isVowel(r) {
			v = append(v, r)
		}
	}
	for i, r := range s {
		if isVowel(r) {
			s[i] = v[len(v)-1]
			v = v[:len(v)-1]
		}
	}
	return string(s)
}

func main() {
	fmt.Println(reverseVowels("aeiIEA"))
}
