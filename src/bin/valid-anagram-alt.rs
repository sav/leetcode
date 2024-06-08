// valid-anagram.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 242. Valid-Anagram
//!
//!  - https://leetcode.com/problems/valid-anagram

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let (mut s_chars, mut t_chars) =
            (s.chars().collect::<Vec<_>>(), t.chars().collect::<Vec<_>>());

        s_chars.sort();
        t_chars.sort();

        let (sorted_s, sorted_t) = (String::from_iter(s_chars), String::from_iter(t_chars));

        sorted_s == sorted_t
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let s = input.read_line_as::<String>()?;
    let t = input.read_line_as::<String>()?;

    println!("{}", Solution::is_anagram(s, t));

    Ok(())
}
