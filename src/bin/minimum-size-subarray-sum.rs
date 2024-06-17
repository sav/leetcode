// minimum-size-subarray-sum.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 209. Minimum Size Subarray Sum
//!
//!  - https://leetcode.com/problems/minimum-size-subarray-sum

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let (mut left, mut right, length) = (0, 0, nums.len());
        let (mut min, mut sum) = (i32::MAX, nums[left]);

        while left < length && right < length {
            if sum < target && right + 1 < length {
                right += 1;
                sum += nums[right];
            } else {
                let size = (right - left + 1) as i32;
                if sum >= target && size < min {
                    min = size;
                }
                sum -= nums[left];
                left += 1;
            }
        }

        if min == i32::MAX {
            min = 0;
        }

        min as i32
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let target = input.read_as::<i32>()?;
    let Vector(nums) = input.read_as::<Vector<i32>>()?;

    let answer = Solution::min_sub_array_len(target, nums);
    println!("{}", answer);

    Ok(())
}
