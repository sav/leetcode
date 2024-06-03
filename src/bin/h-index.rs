// h-index.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 274. H-Index
//!
//!  - https://leetcode.com/problems/h-index

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn h_index(mut v: Vec<i32>) -> i32 {
        v.sort_by(|a, b| b.cmp(a));

        let n = v.len();
        let mut i = 0;

        while i < n && v[i] > i as i32 {
            i += 1;
        }

        i as i32
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let Vector(nums) = input.read_line_as::<Vector<i32>>()?;

    println!("{:?}", nums);
    let h = Solution::h_index(nums);
    println!("{}", h);

    Ok(())
}
