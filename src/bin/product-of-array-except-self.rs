// product-of-array-except-self.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 238. Product of Array Except Self
//!
//!  - https://leetcode.com/problems/product-of-array-except-self

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    #[inline]
    fn _product_except_self(v: &mut Vec<i32>, i: usize, left: i32) -> i32 {
        let current = v[i];
        if i == v.len() - 1 {
            v[i] = left;
            return current;
        }

        let right = Self::_product_except_self(v, i + 1, left * current);
        v[i] = left * right;

        current * right
    }

    pub fn product_except_self(mut v: Vec<i32>) -> Vec<i32> {
        Self::_product_except_self(&mut v, 0, 1);
        v
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let Vector(v) = input.read_line_as::<Vector<i32>>()?;

    println!("{v:?}");
    let w = Solution::product_except_self(v);
    println!("{w:?}");

    Ok(())
}
