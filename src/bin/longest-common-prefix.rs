// longest-common-prefix.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 14. Longest Common Prefix
//!
//!  - https://leetcode.com/problems/longest-common-prefix

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strings: Vec<String>) -> String {
        if strings.len() < 1 {
            return "".to_string();
        }

        let mut prefix_len = strings[0].len();
        let prefix = strings[0].clone().chars().collect::<Vec<char>>();

        for i in 1..strings.len() {
            let chars = strings[i].chars().collect::<Vec<char>>();
            let mut last = 0;

            for j in 0..prefix_len.min(strings[i].len()) {
                if prefix[j] == chars[j] {
                    last = j + 1;
                } else {
                    break;
                }
            }

            if last < prefix_len {
                prefix_len = last;
            }

            if prefix_len == 0 {
                break;
            }
        }

        String::from_iter(&prefix[0..prefix_len])
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let Vector(v) = input.read_line_as::<Vector<String>>()?;

    let r = Solution::longest_common_prefix(v);
    println!("{}", r);

    Ok(())
}
