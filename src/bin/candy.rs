// candy.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 135. Candy
//!
//!  - https://leetcode.com/problems/candy

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn candy(v: Vec<i32>) -> i32 {
        let n = v.len();
        let mut w = vec![1; n];

        for i in 1..n {
            if v[i] > v[i - 1] {
                w[i] = w[i - 1] + 1
            }
        }

        for i in (0..n - 1).rev() {
            if v[i] > v[i + 1] {
                w[i] = w[i].max(w[i + 1] + 1)
            }
        }

        w.iter().sum::<i32>() as i32
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let Vector(nums) = input.read_line_as::<Vector<i32>>()?;

    println!("{:?}", nums);
    let h = Solution::candy(nums);
    println!("{}", h);

    Ok(())
}
