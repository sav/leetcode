// longest-consecutive-sequence.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 128. Longest Consecutive Sequence
//!
//!  - https://leetcode.com/problems/longest-consecutive-sequence

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }

        nums.sort();

        let (mut max, mut acc) = (0, 0);

        for i in 0..nums.len() - 1 {
            if nums[i] + 1 == nums[i + 1] {
                acc += 1;
                if acc > max {
                    max = acc;
                }
            } else if nums[i] != nums[i + 1] {
                acc = 0;
            }
        }

        max + 1
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let Vector(nums) = input.read_line_as::<Vector<i32>>()?;

    let answer = Solution::longest_consecutive(nums);
    println!("{}", answer);

    Ok(())
}
