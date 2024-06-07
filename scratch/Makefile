##
# Leetcode Scraps
#
# @file scrap/Makefile
# @version 0.1

LEETCODE_GO := leetcode.go

LEETCODE_GO_BINS := \
	valid-palindrome \
	is-subsequence \
	two-sum-ii-input-array-is-sorted

all: $(LEETCODE_GO_BINS)

valid-palindrome: valid-palindrome.go $(LEETCODE_GO)
	@go build valid-palindrome.go $(LEETCODE_GO)

is-subsequence: is-subsequence.go $(LEETCODE_GO)
	@go build is-subsequence.go $(LEETCODE_GO)

two-sum-ii-input-array-is-sorted: two-sum-ii-input-array-is-sorted.go $(LEETCODE_GO)
	@go build two-sum-ii-input-array-is-sorted.go $(LEETCODE_GO)

.PHONY: clean
clean:
	@rm -f $(LEETCODE_GO_BINS)

# end
