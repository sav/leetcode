package main

func canConstruct(note string, magazine string) bool {
	m := make(map[rune]int)

	for _, c := range magazine {
		m[c] += 1
	}

	for _, c := range note {
		if m[c] < 1 {
			return false
		}
		m[c] -= 1
	}

	return true
}

func main() {
	Assert(canConstruct, "a", "b", false)
	Assert(canConstruct, "abc", "banana caramba", true)
}
