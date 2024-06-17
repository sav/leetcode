// valid-sudoku.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 36. Valid Sudoku
//!
//!  - https://leetcode.com/problems/valid-sudoku

use leetcode::*;
use std::error::Error;

struct Solution;

fn is_valid_seq(seq: &Vec<char>) -> bool {
    let mut has: Vec<i8> = vec![0; 10];
    for &x in seq {
        if x == '.' || x == '0' {
            continue;
        }
        let x = (x as usize) - ('0' as usize);
        has[x] += 1;
        if has[x] > 1 {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn is_valid_sudoku(m: Vec<Vec<char>>) -> bool {
        for n in 0..9 {
            if !is_valid_seq(&m[n]) {
                return false;
            }
            if !is_valid_seq(&m.iter().map(|row| row[n]).collect()) {
                return false;
            }
            let mut s = vec![];
            for i in (n / 3 * 3)..(n / 3 * 3 + 3) {
                for j in ((n % 3) * 3)..((n % 3) * 3 + 3) {
                    s.push(m[i][j]);
                }
            }
            if !is_valid_seq(&s) {
                return false;
            }
        }

        true
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let sudoku: Vec<Vec<_>> = input.read_matrix_of::<char>()?;

    let answer = Solution::is_valid_sudoku(sudoku);
    println!("{}", answer);

    Ok(())
}
