// length-of-last-word.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 58. Length of Last Word
//!
//!  - https://leetcode.com/problems/length-of-last-word

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim().split(' ').rev().next().unwrap().len() as i32
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let s = input.read_as::<String>()?;

    let n = Solution::length_of_last_word(s);
    println!("{}", n);

    Ok(())
}
