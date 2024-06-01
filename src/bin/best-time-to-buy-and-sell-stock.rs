// best-time-to-buy-and-sell-stock.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 121. Best Time to Buy and Sell Stock
//!
//!  - https://leetcode.com/problems/best-time-to-buy-and-sell-stock

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn max_profit(v: Vec<i32>) -> i32 {
        let (mut min_price, mut max_profit) = (i32::MAX, 0);
        let n = v.len();

        for i in 0..n {
            let price = v[i];
            if price < min_price {
                min_price = price;
            } else if price - min_price > max_profit {
                max_profit = price - min_price;
            }
        }

        max_profit
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let Vector(prices) = input.read_line_as::<Vector<i32>>()?;

    println!("{prices:?}");
    let day = Solution::max_profit(prices);
    println!("{day}");

    Ok(())
}
