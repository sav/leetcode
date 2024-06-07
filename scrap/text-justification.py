#!/usr/bin/env python3

import pprint


class Solution:
    def format_line(self, words: list[str], width: int) -> str:
        n = len(words)

        if n < 2:
            return words[0] + " " * (width - len(words[0]))

        length = sum(map(len, words)) + n - 1
        extra = width - length
        per_word = 1 + (extra // (n - 1))
        remainder = extra - (extra // (n - 1)) * (n - 1)

        line = ""

        for i in range(n - 1):
            spaces = " " * per_word
            if remainder > 0:
                spaces += " "
                remainder -= 1
            line += words[i] + spaces

        line += words[n - 1]

        return line

    def fullJustify(self, words: list[str], width: int) -> list[str]:
        next_word, curr_words, curr_len, lines = 0, [], 0, []

        while True:
            word = words[next_word]

            if word == "to":
                print(f"curr_len={curr_len}, word={word}, curr_words={curr_words}")

            # The `width + 1` below is necessary because we're always summing up
            # head on to `curr_len`.
            if curr_len + 1 + len(word) <= width + 1:
                curr_words += [word]
                curr_len += 1 + len(word)
                next_word += 1
            else:
                lines += [curr_words]
                curr_words = []
                curr_len = 0

            if next_word == len(words):
                break

        if len(curr_words) > 0:
            lines += [curr_words]

        # Create a line out of the remaining words (which lengths hasn't summed
        # up to the width yet), if any.
        for i in range(len(lines) - 1):
            lines[i] = self.format_line(lines[i], width)

        # Left-justify the last line.
        lines[-1] = " ".join(lines[-1])
        lines[-1] += " " * (width - len(lines[-1]))

        return lines


def assert_lengths(lines, width):
    for line in lines:
        assert (
            len(line) == width
        ), f"error: length is {len(line)}, should be {width}: {line}"
    return lines


ex1 = {
    "words": ["This", "is", "an", "example", "of", "text", "justification."],
    "width": 16,
}

ex2 = {
    "words": ["What", "must", "be", "acknowledgment", "shall", "be"],
    "width": 16,
}

ex3 = {
    "words": [
        "Science",
        "is",
        "what",
        "we",
        "understand",
        "well",
        "enough",
        "to",
        "explain",
        "to",
        "a",
        "computer.",
        "Art",
        "is",
        "everything",
        "else",
        "we",
        "do",
    ],
    "width": 20,
}

sol = Solution()

pprint.pprint(assert_lengths(sol.fullJustify(**ex1), ex1["width"]))
pprint.pprint(assert_lengths(sol.fullJustify(**ex2), ex2["width"]))
pprint.pprint(assert_lengths(sol.fullJustify(**ex3), ex3["width"]))
