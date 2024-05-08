// remove-element.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 27. Remove Element
//!
//!  - https://leetcode.com/problems/remove-element

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    /// Given an integer array `nums` and an integer `val`, remove all
    /// occurrences of `val` in `nums` *in-place*. The order of the elements
    /// may be changed. Then return the number of elements in `nums` which are
    /// not equal to `val`.
    ///
    /// Consider the number of elements in `nums` which are not equal to `val`
    /// be `k`, to get accepted, you need to do the following things:
    ///
    ///  - Change the array nums such that the first `k` elements of `nums`
    ///    contain the elements which are not equal to `val`. The remaining
    ///    elements of `nums` are not important as well as the size of `nums`.
    ///  - Return `k`.
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0_i32;
        let mut p = -1_i32;
        
        for i in 0..nums.len() {
            if nums[i] != val {
                if p != -1 {
                    nums[p as usize] = nums[i];
                    nums[i] = -1;
                    p += 1;
                }
                k += 1;
            } else {
                nums[i] = -1;
                if p == -1 {
                    p = i as i32;
                }
            }
        }
        k
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let Vector(mut nums) = input.read_line_as::<Vector<i32>>()?;
    let val: i32 = input.read_line_as()?;

    let result = Solution::remove_element(&mut nums, val);

    println!("{result}\n{nums:?}");
    
    Ok(())
}
