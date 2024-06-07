// reverse-words-in-a-string.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 151. Reverse Words in a String
//!
//!  - https://leetcode.com/problems/reverse-words-in-a-string

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.trim()
            .split(' ')
            .rev()
            .map(|s| s.trim())
            .filter(|x| x.len() > 0)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let s = input.read_line_as::<String>()?;

    println!("{s}");
    let r = Solution::reverse_words(s);
    println!("{r}");

    Ok(())
}
