// rotate-array.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 189. Rotate Array
//!
//!  - https://leetcode.com/problems/rotate-array

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn rotate(v: &mut Vec<i32>, k: i32) {
        let n = v.len();
        let k = k as usize % n;
        let w: Vec<i32> = v[n - k..n].to_vec();

        for i in (0..n - k).rev() {
            v[k + i] = v[i];
        }

        for i in 0..k {
            v[i] = w[i];
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let Vector(mut nums) = input.read_as::<Vector<i32>>()?;
    let k: i32 = input.read_as()?;

    Solution::rotate(&mut nums, k);
    println!("{:?}", nums);

    Ok(())
}
