// game-of-life.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 289. Game Of Life
//!
//!  - https://leetcode.com/problems/game-of-life

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn game_of_life(b: &mut Vec<Vec<i32>>) {
        let (n, m) = (b.len(), b[0].len());
        let (mut kill, mut bear) = (vec![], vec![]);

        for i in 0..n {
            for j in 0..m {
                let mut live = 0;

                if i > 0 && b[i - 1][j] > 0 {
                    live += 1;
                }
                if i > 0 && j + 1 < m && b[i - 1][j + 1] > 0 {
                    live += 1;
                }
                if j + 1 < m && b[i][j + 1] > 0 {
                    live += 1;
                }
                if i + 1 < n && j + 1 < m && b[i + 1][j + 1] > 0 {
                    live += 1;
                }
                if i + 1 < n && b[i + 1][j] > 0 {
                    live += 1;
                }
                if i + 1 < n && j > 0 && b[i + 1][j - 1] > 0 {
                    live += 1;
                }
                if j > 0 && b[i][j - 1] > 0 {
                    live += 1;
                }
                if i > 0 && j > 0 && b[i - 1][j - 1] > 0 {
                    live += 1;
                }

                // Live cell with fewer than two live neighbors dies by under-population.
                if b[i][j] == 1 && live < 2 {
                    kill.push((i, j));
                }
                // Live cell with more than three live neighbors dies by over-population.
                if b[i][j] == 1 && live > 3 {
                    kill.push((i, j));
                }
                // Dead cell with exactly three live neighbors becomes a live cell.
                if b[i][j] == 0 && live == 3 {
                    bear.push((i, j));
                }
            }
        }

        for (i, j) in &kill {
            b[*i][*j] = 0;
        }
        for (i, j) in &bear {
            b[*i][*j] = 1;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let mut matrix = input.read_matrix_of::<i32>()?;

    Solution::game_of_life(&mut matrix);
    println!("{:?}", matrix);

    Ok(())
}
