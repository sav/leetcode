##
# Leetcode Scraps
#
# @file scrap/Makefile
# @version 0.1

LEETCODE_GO := leetcode.go

all: valid-palindrome is-subsequence

valid-palindrome: valid-palindrome.go $(LEETCODE_GO)
	@go build valid-palindrome.go $(LEETCODE_GO)

is-subsequence: is-subsequence.go $(LEETCODE_GO)
	@go build is-subsequence.go $(LEETCODE_GO)

.PHONY: clean
clean:
	@rm -f valid-palindrome

# end
