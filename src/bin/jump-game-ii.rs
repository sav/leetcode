// jump-game-ii.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 45. Jump Game II
//!
//!  - https://leetcode.com/problems/jump-game-ii

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn jump(v: Vec<i32>) -> i32 {
        let last = v.len() - 1;
        let mut jumps = 0;
        let mut i = 0;

        while i < last {
            jumps += 1;
            // If we can jump directly to the end, do so.
            if i + v[i] as usize >= last {
                break;
            }
            // Otherwise, discover within the range of possible destinations,
            // which one will maximize the distance traveled.
            let (ini, end) = (i + 1, (i + v[i] as usize).min(last));
            let (offset, _) = v[ini..end + 1].iter().enumerate().fold((0, 0), |acc, v| {
                if v.0 + *v.1 as usize >= acc.1 {
                    // if i + v[i] >= max
                    (v.0, v.0 + *v.1 as usize) //    (i, i + v[i])
                } else {
                    acc
                }
            });
            i += offset + 1;
        }

        jumps
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let Vector(nums) = input.read_as::<Vector<i32>>()?;

    println!("{:?}", nums);
    let result = Solution::jump(nums);
    println!("{}", result);

    Ok(())
}
