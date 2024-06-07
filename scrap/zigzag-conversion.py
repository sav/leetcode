#!/usr/bin/env python3

class Solution:
    def convert(self, s: str, n: int) -> str:
        if n < 2:
            return s
        
        outstr = ["" for _ in range(n)]
        level = 0
        direction = False

        for i in range(len(s)):
            outstr[level] += s[i:i+1]
            
            if i % (n - 1) == 0:
                direction = not direction
                
            level = level + 1 if direction else level - 1

        return "".join(outstr)


leet = Solution()

print(" n=2 ->", leet.convert("0123456789", 2))
print(" n=3 ->", leet.convert("0123456789", 3))
print(" n=4 ->", leet.convert("0123456789", 4))
print(" n=5 ->", leet.convert("0123456789", 5))
print(" n=6 ->", leet.convert("0123456789", 6))
print(" n=7 ->", leet.convert("0123456789", 7))
print(" n=8 ->", leet.convert("0123456789", 8))
print(" n=9 ->", leet.convert("0123456789", 9))
print("n=10 ->", leet.convert("0123456789", 10))
