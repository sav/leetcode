// remove-duplicates-from-sorted-array-ii.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

#![allow(dead_code)]

//! # 80. Remove Duplicates from Sorted Array II
//!
//!  - https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii

use leetcode::*;
use std::collections::HashMap;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut h: HashMap<i32, i32> = HashMap::new();
        let mut i = 0 as usize;

        while i < nums.len() {
            let c = h.get(&nums[i]).unwrap_or(&0);
            if *c < 2 {
                h.insert(nums[i], *c + 1);
                i += 1;
            } else {
                nums.remove(i);
            }
        }

        nums.len() as i32
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let Vector(mut nums) = input.read_as::<Vector<i32>>()?;

    let result = Solution::remove_duplicates(&mut nums);

    println!("{result}\n{nums:?}");

    Ok(())
}
