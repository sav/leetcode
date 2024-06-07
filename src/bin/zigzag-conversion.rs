// zigzag-conversion.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 6. Zigzag Conversion
//!
//!  - https://leetcode.com/problems/zigzag-conversion

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn convert(s: String, n: i32) -> String {
        if n < 2 {
            return s;
        }

        let n = n as usize;
        let mut levels = vec![String::from(""); n as usize];
        let mut level = 0_i32;
        let mut up_or_down = false;

        for i in 0..s.len() {
            levels[level as usize] += s.get(i..i+1).unwrap();

            if i % (n - 1) == 0 {
                up_or_down = !up_or_down;
            }

            level += if up_or_down { 1 } else { -1 };
        }

        levels.join("")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let s = input.read_line_as::<String>()?;
    let n = input.read_line_as::<i32>()?;

    let r = Solution::convert(s, n);
    println!("{r}");

    Ok(())
}
