##
# Leetcode Scraps
#
# @file scrap/Makefile
# @version 0.1

GO := go

LEETCODE_GO := leetcode.go

LEETCODE_GO_BINS := \
	valid-palindrome \
	is-subsequence \
	two-sum-ii-input-array-is-sorted \
	3sum

all: $(LEETCODE_GO_BINS) 

$(LEETCODE_GO_BINS): %: %.go $(LEETCODE_GO)
	$(GO) build $@.go $(LEETCODE_GO)

.PHONY: clean
clean:
	rm -f $(LEETCODE_GO_BINS)

# end
