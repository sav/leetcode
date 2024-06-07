package main

func isSubsequence(s string, t string) bool {
	n := len(t)
	m := len(s)

	found := 0

	for i := 0; i < n && found < m; i++ {
		if s[found] == t[i] {
			found++
		}
	}

	return found == m
}

func main() {
	Assert(isSubsequence, "abc", "ahbgdc", true)
	Assert(isSubsequence, "axc", "ahbgabcdc", false)
	Assert(isSubsequence, "", "ahbgabcdc", true)
}
