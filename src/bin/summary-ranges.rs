// summary-ranges.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 228. Summary Ranges
//!
//!  - https://leetcode.com/problems/summary-ranges

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn summary_ranges(v: Vec<i32>) -> Vec<String> {
        let (mut w, mut last) = (vec![], v[0] - 1);
        let mut ret = vec![];

        let mut range = |w: &Vec<i32>| {
            if w.len() == 1 {
                ret.push(format!("{}", w[0]));
            } else {
                ret.push(format!("{}->{}", w[0], w[w.len() - 1]));
            }
        };

        for &e in &v {
            if e == last + 1 {
                w.push(e);
            } else {
                range(&w);
                w.clear();
                w.push(e);
            }
            last = e;
        }
        range(&w);

        ret
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let Vector(nums) = input.read_as::<Vector<i32>>()?;

    let answer = Solution::summary_ranges(nums);
    println!("{:?}", answer);

    Ok(())
}
