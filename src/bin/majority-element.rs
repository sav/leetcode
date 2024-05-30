// majority-element.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 169. Majority Element
//!
//!  - https://leetcode.com/problems/majority-element

use leetcode::*;
use std::collections::HashMap;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut h: HashMap<i32, i32> = HashMap::new();
        for x in &nums {
            h.insert(*x, h.get(x).unwrap_or(&0) + 1);
        }
        let (mut ret, mut cnt) = (i32::MIN, i32::MIN);
        for (k, v) in h.iter() {
            if *v > cnt {
                (ret, cnt) = (*k, *v);
            }
        }
        ret
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let Vector(nums) = input.read_line_as::<Vector<i32>>()?;

    println!("{:?}", nums);

    let n = Solution::majority_element(nums);

    println!("{}", n);

    Ok(())
}
