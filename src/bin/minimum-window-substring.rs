// minimum-window-substring.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 76. Minimum Window Substring
//!
//!  - https://leetcode.com/problems/minimum-window-substring

use leetcode::*;
use std::collections::HashMap;
use std::error::Error;

struct Solution;

fn is_complete(map1: &HashMap<char, i32>, map2: &HashMap<char, i32>) -> bool {
    for (key, &val) in map2 {
        if let Some(&other) = map1.get(key) {
            if other >= val {
                continue;
            }
        }
        return false;
    }
    true
}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.len() > s.len() {
            return "".to_string();
        }

        let mut char_map: HashMap<char, i32> = HashMap::with_capacity(t.len());
        for c in t.chars() {
            *char_map.entry(c).or_insert(0) += 1;
        }

        #[derive(Debug)]
        struct Found {
            i: usize,
            c: char,
        }

        let mut found: Vec<Found> = Vec::with_capacity(s.len());
        for (i, c) in s.chars().enumerate() {
            if let Some(&_) = char_map.get(&c) {
                found.push(Found { i, c });
            }
        }

        let n_found = found.len();
        if n_found < t.len() {
            return "".to_string();
        }

        let mut window_map: HashMap<char, i32> = HashMap::with_capacity(t.len());
        let (mut best, mut best_size) = ("", usize::MAX);
        let mut left = 0;

        for right in 0..n_found {
            let c = found[right].c;
            let n = window_map.entry(c).or_insert(0);

            *n += 1;

            let mut m = n;
            let mut d = c;

            while found[left].c == d && *m > char_map[&d] {
                left += 1;
                *m -= 1;
                d = found[left].c;
                m = window_map.entry(d).or_insert(0);
            }

            let window_size = found[right].i - found[left].i + 1;
            if window_map[&c] >= char_map[&c]
                && window_size < best_size
                && is_complete(&window_map, &char_map)
            {
                best = &s[found[left].i..found[right].i + 1];
                best_size = window_size;
            }
        }

        best.to_string()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let s = input.read_as::<String>()?;
    let t = input.read_as::<String>()?;

    println!("{s} {t}");
    let answer = Solution::min_window(s, t);
    println!("{answer:#?} ({})", answer.len());

    Ok(())
}
