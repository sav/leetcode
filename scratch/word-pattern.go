//go:build ignore

package main

import (
	"strings"
)

func wordPattern(pattern string, s string) bool {
	words := strings.Split(s, " ")

	patternRunes := []rune(pattern)

	if len(words) != len(patternRunes) {
		return false
	}

	patternToWord := make(map[rune]string)
	wordToPattern := make(map[string]rune)

	for i, word := range words {
		currWord, wordExists := patternToWord[patternRunes[i]]
		if wordExists && word != currWord {
			return false
		} else if !wordExists {
			patternToWord[patternRunes[i]] = word
		}

		currPattern, patternExists := wordToPattern[word]
		if patternExists && patternRunes[i] != currPattern {
			return false
		} else if !patternExists {
			wordToPattern[word] = patternRunes[i]
		}
	}

	return true
}

func main() {
	Assert(wordPattern, "abba", "cat dog dog cat", true)
	Assert(wordPattern, "abbr", "cat dog dog cat", false)
}
