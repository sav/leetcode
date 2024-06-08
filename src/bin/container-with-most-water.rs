// container-with-most-water.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 11. Container With Most Water
//!
//!  - https://leetcode.com/problems/container-with-most-water

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let length = height.len();
        let (mut total, mut left, mut right) = (0, 0usize, length - 1);

        while left < right {
            let area = height[left].min(height[right]) * (right - left) as i32;
            
            if area > total {
                total = area
            }

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        total
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let Vector(height) = input.read_line_as::<Vector<i32>>()?;

    let area = Solution::max_area(height);
    println!("{area}");

    Ok(())
}
