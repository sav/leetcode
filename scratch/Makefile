##
# Leetcode Scraps
#
# @file scrap/Makefile
# @version 0.1

CPP := clang++

CXXFLAGS += --std=c++20 -g -O2 -Wall 

GO := go

LEETCODE_GO := leetcode.go

LEETCODE_GO_BINS := \
	valid-palindrome \
	is-subsequence \
	two-sum-ii-input-array-is-sorted \
	3sum \
	ransom-note \
	isomorphic-strings \
	word-pattern

LEETCODE_CPP := leetcode.hpp

LEETCODE_CPP_BINS := \
	happy-number \
	2sum \
	contains-duplicate-ii

all: $(LEETCODE_GO_BINS) $(LEETCODE_CPP_BINS) 

$(LEETCODE_GO_BINS): %: %.go $(LEETCODE_GO)
	$(GO) build $@.go $(LEETCODE_GO)

$(LEETCODE_CPP_BINS): %: %.cpp $(LEETCODE_CPP)
	$(CPP) $(CXXFLAGS) -o $@ $@.cpp

.PHONY: clean
clean:
	rm -f $(LEETCODE_GO_BINS) $(LEETCODE_CPP_BINS)

# end
