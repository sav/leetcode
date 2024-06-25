// set-matrix-zeroes.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 73. Set Matrix Zeroes
//!
//!  - https://leetcode.com/problems/set-matrix-zeroes

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn set_zeroes(a: &mut Vec<Vec<i32>>) {
        let (m, n) = (a.len(), a[0].len());
        let (mut rows, mut cols) = (vec![0; m], vec![0; n]);

        for i in 0..m {
            for j in 0..n {
                if a[i][j] == 0 {
                    (rows[i], cols[j]) = (1, 1);
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                if rows[i] == 1 || cols[j] == 1 {
                    a[i][j] = 0;
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let mut matrix = leetcode::Matrix::<i32>(input.read_matrix_of::<i32>()?);

    println!("{}", matrix);
    Solution::set_zeroes(&mut matrix.0);
    println!("{}", matrix);

    Ok(())
}
