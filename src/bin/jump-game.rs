// jump-game.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 55. Jump Game
//!
//!  - https://leetcode.com/problems/jump-game

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn can_jump(v: Vec<i32>) -> bool {
        let last = v.len() - 1;
        let mut jump = 0;

        for i in 0..last {
            if v[i] > jump {
                jump = v[i];
            }
            if v[i] < 1 && jump < 1 {
                return false;
            }
            jump -= 1;
        }

        true
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let Vector(nums) = input.read_as::<Vector<i32>>()?;

    println!("{:?}", nums);

    let result = Solution::can_jump(nums);

    println!("{}", result);

    Ok(())
}
