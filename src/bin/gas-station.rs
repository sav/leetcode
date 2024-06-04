// gas-station.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 134. Gas Station
//!
//!  - https://leetcode.com/problems/gas-station
//!

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn can_complete_circuit(mut gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut sum = 0;

        // Zip `gas` and `cost` into `gas - cost`.
        for i in (0..gas.len()).rev() {
            gas[i] -= cost[i];
            sum += gas[i];
        }

        // Not enough gas, return.
        if sum < 0 {
            return -1;
        } else {
            sum = 0;
        }

        let (mut max, mut max_pos) = (i32::MIN, -1);

        // Sum up the effective `gas - cost` and remembert the maximum.
        for i in (0..gas.len()).rev() {
            sum += gas[i];
            if sum > max {
                max = sum;
                max_pos = i as i32;
            }
        }

        max_pos
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let Vector(gas) = input.read_line_as::<Vector<i32>>()?;
    let Vector(cost) = input.read_line_as::<Vector<i32>>()?;

    // println!("{gas:?}\n{cost:?}");
    let n = Solution::can_complete_circuit(gas, cost);
    println!("{n:?}");

    Ok(())
}
