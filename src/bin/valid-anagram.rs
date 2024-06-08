// valid-anagram.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 242. Valid-Anagram
//!
//!  - https://leetcode.com/problems/valid-anagram

use leetcode::*;
use std::collections::HashMap;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_map: HashMap<char, i32> = HashMap::with_capacity(s.len());
        let mut t_map: HashMap<char, i32> = HashMap::with_capacity(s.len());

        for (sc, tc) in s.chars().zip(t.chars()) {
            s_map.insert(sc, s_map.get(&sc).unwrap_or(&0) + 1);
            t_map.insert(tc, t_map.get(&tc).unwrap_or(&0) + 1);
        }

        for (s_key, &s_count) in s_map.iter() {
            if let Some(&t_count) = t_map.get(s_key) {
                if s_count != t_count {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let s = input.read_line_as::<String>()?;
    let t = input.read_line_as::<String>()?;

    println!("{}", Solution::is_anagram(s, t));

    Ok(())
}
