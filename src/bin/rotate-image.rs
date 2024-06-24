// rotate-image.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 48. Rotate Image
//!
//!  - https://leetcode.com/problems/rotate-image

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn rotate(a: &mut Vec<Vec<i32>>) {
        let n = a.len();
        for k in 0..n / 2 {
            let m = n - k - 1;
            for p in 0..m - k {
                let t1 = a[k][k + p];
                let t2 = a[k + p][m];
                let t3 = a[m][m - p];
                let t4 = a[m - p][k];
                a[k][k + p] = t4;
                a[k + p][m] = t1;
                a[m][m - p] = t2;
                a[m - p][k] = t3;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let matrix = input.read_matrix_of::<i32>()?;
    let mut matrix = leetcode::Matrix(matrix);

    println!("{}", matrix);
    Solution::rotate(&mut matrix.0);
    println!("{}", matrix);

    Ok(())
}
