// remove-duplicates-from-sorted-array.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

#![allow(dead_code, unused_variables, unused_mut)]

//! # 26. Remove Duplicates from Sorted Array
//!
//!  - https://leetcode.com/problems/remove-duplicates-from-sorted-array

use leetcode::*;
use std::collections::HashSet;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut map: HashSet<i32> = HashSet::new();
        let mut i = 0 as usize;

        for j in 0..nums.len() {
            if !map.contains(&nums[j]) {
                nums[i] = nums[j];
                i += 1;
                map.insert(nums[j]);
            }
        }

        i as i32
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let Vector(mut nums) = input.read_line_as::<Vector<i32>>()?;

    let result = Solution::remove_duplicates(&mut nums);

    println!("{result}\n{nums:?}");

    Ok(())
}
