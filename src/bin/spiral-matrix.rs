// spiral-matrix.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 54. Spiral Matrix
//!
//!  - https://leetcode.com/problems/spiral-matrix

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn spiral_order(a: Vec<Vec<i32>>) -> Vec<i32> {
        if a.len() < 1 || a[0].len() < 1 {
            return vec![];
        }

        let mut r = vec![];
        let (n, m) = (a.len(), a[0].len());
        let (mut top, mut bottom, mut left, mut right) = (0, a.len() - 1, 0, a[0].len() - 1);
        let (mut i, mut j) = (0, 0);
        let mut dir = 0; // 0=right, 1=down, 2=left, 3=up

        while i < n && j < m && top <= bottom && left <= right {
            r.push(a[i][j]);

            if dir == /*right*/ 0 {
                if j < right {
                    j += 1;
                } else {
                    i += 1;
                    dir = 1;
                    top += 1;
                }
            } else if dir == /*down*/ 1 {
                if i < bottom {
                    i += 1;
                } else {
                    if j as i32 - 1 < 0 {
                        break;
                    }
                    j -= 1;
                    dir = 2;
                    right -= 1;
                }
            } else if dir == /*left*/ 2 {
                // left
                if j > left {
                    j -= 1;
                } else {
                    if i as i32 - 1 < 0 {
                        break;
                    }
                    i -= 1;
                    dir = 3;
                    bottom -= 1;
                }
            } else if dir == /*up*/ 3 {
                if i > top {
                    i -= 1;
                } else {
                    j += 1;
                    dir = 0;
                    left += 1;
                }
            }
        }

        r
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let matrix = input.read_matrix_of::<i32>()?;

    let answer = Solution::spiral_order(matrix);
    println!("{:?}", answer);

    Ok(())
}
