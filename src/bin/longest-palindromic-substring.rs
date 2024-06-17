// longest-palindromic-substring.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 5. Longest Palindromic Substring
//!
//!  - https://leetcode.com/problems/longest-palindromic-substring

use leetcode::*;
use std::error::Error;

struct Solution;

const SEP: char = '|';

#[inline]
fn preprocess(s: String) -> String {
    let mut r = String::new();

    r.push(SEP);
    for c in s.chars() {
        r.push(c);
        r.push(SEP);
    }

    r
}

#[inline]
fn postprocess(chars: Vec<char>, center: i32, radius: i32) -> String {
    if center < 0 || radius == 0 {
        return "".to_string();
    }

    let ini = (center - radius) as usize;
    let end = (center + radius + 1) as usize;

    chars[ini..end].iter().filter(|&c| *c != SEP).collect()
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = preprocess(s); // convert `abc` to `|a|b|c|`.

        let chars: Vec<char> = s.chars().collect();
        let length = chars.len() as i32;

        let mut center = 0;
        let mut max_radius = 0;
        let mut max_center = -1;

        while center < length {
            let mut radius = 0;

            while center - (radius + 1) >= 0
                && center + (radius + 1) < length
                && chars[(center - (radius + 1)) as usize]
                    == chars[(center + (radius + 1)) as usize]
            {
                radius += 1;
            }

            if radius > max_radius {
                max_radius = radius;
                max_center = center;
            }

            center += 1;
        }

        postprocess(chars, max_center, max_radius)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();
    let s = input.read_as::<String>()?;
    let p = Solution::longest_palindrome(s.clone());
    println!("{p}");
    Ok(())
}
