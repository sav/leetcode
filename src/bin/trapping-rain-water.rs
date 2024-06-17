// trapping-rain-water.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 42. Trapping Rain Water
//!
//!  - https://leetcode.com/problems/trapping-rain-water

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    fn get_right_max(v: &Vec<i32>) -> Vec<i32> {
        let mut w = vec![0; v.len()];
        let mut max = 0;
        for i in (0..v.len() - 1).rev() {
            if v[i + 1] > max {
                max = v[i + 1]
            }
            w[i] = max;
        }
        w
    }

    pub fn trap(v: Vec<i32>) -> i32 {
        let right_max = Solution::get_right_max(&v);

        let mut trapped = 0;
        let mut left_max = 0;

        for i in 0..v.len() {
            if left_max > v[i] && right_max[i] > v[i] {
                let height = left_max.min(right_max[i]);
                trapped += height - v[i];
            }
            if v[i] > left_max {
                left_max = v[i];
            }
        }

        trapped
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let Vector(nums) = input.read_as::<Vector<i32>>()?;

    println!("{:?}", nums);
    let h = Solution::trap(nums);
    println!("{}", h);

    Ok(())
}
