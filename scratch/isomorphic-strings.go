package main

func isIsomorphic(s string, t string) bool {
	length := len(s)

	if length != len(t) {
		return false
	}

	mapStoT := make(map[rune]rune)
	mapTtoS := make(map[rune]rune)

	sr := []rune(s)
	tr := []rune(t)

	for i := 0; i < length; i++ {
		sChar, sExists := mapStoT[sr[i]]

		if sExists && sChar != tr[i] {
			return false
		} else if !sExists {
			mapStoT[sr[i]] = tr[i]
		}

		tChar, tExists := mapTtoS[tr[i]]

		if tExists && tChar != sr[i] {
			return false
		} else if !tExists {
			mapTtoS[tr[i]] = sr[i]
		}

	}

	return true
}

func main() {
	Assert(isIsomorphic, "egg", "add", true)
	Assert(isIsomorphic, "pad", "add", false)
}
