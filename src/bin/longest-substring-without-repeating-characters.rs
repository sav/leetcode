// longest-substring-without-repeating-characters.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 3. Longest Substring Without Repeating Characters
//!
//!  - https://leetcode.com/problems/longest-substring-without-repeating-characters

use leetcode::*;
use std::collections::HashMap;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let n = s.len();
        if n < 2 {
            return n as i32;
        }

        let s = s.chars().collect::<Vec<char>>();
        let mut map: HashMap<char, usize> = HashMap::new();

        let (mut best, mut first, mut left, mut right) = (0usize, 0usize, 0usize, 0usize);

        while left < n && right < n {
            if let Some(&pos) = map.get(&s[right]) {
                // remove released characters from the map.
                for i in first..pos {
                    map.remove(&s[i]);
                }
                // update the position of the current character.
                map.insert(s[right], right);

                first = pos + 1;
                left = first;
            } else if right - left + 1 > best {
                best = right - left + 1;
            }

            map.insert(s[right], right);
            right += 1;
        }

        best as i32
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let s = input.read_line_as::<String>()?;

    let answer = Solution::length_of_longest_substring(s);
    println!("{}", answer);

    Ok(())
}
