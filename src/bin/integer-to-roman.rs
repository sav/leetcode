// integer-to-roman.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 12. Integer to Roman
//!
//!  - https://leetcode.com/problems/integer-to-roman

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let vals = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let syms = vec![
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];

        let mut s = String::with_capacity(16);

        for i in 0..vals.len() {
            while num - vals[i] >= 0 {
                num -= vals[i];
                s += &syms[i];
            }
        }

        s
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let n = input.read_line_as::<i32>()?;

    let s = Solution::int_to_roman(n);
    println!("{}", s);

    Ok(())
}
