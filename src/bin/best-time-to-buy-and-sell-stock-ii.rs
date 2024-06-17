// best-time-to-buy-and-sell-stock-ii.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 122. Best Time to Buy and Sell Stock II
//!
//!  - https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn max_profit(v: Vec<i32>) -> i32 {
        let n = v.len();
        let mut profit = 0;

        for i in 0..n - 1 {
            if v[i + 1] - v[i] > 0 {
                profit += v[i + 1] - v[i];
            }
        }

        profit
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let Vector(prices) = input.read_as::<Vector<i32>>()?;

    println!("{prices:?}");
    let day = Solution::max_profit(prices);
    println!("{day}");

    Ok(())
}
