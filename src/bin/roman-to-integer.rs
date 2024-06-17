// roman-to-integer.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 13. Roman to Integer
//!
//!  - https://leetcode.com/problems/roman-to-integer

use leetcode::*;
use std::collections::HashMap;
use std::error::Error;

struct Solution;

impl Solution {
    #[inline]
    fn roman_to_int_map() -> HashMap<char, i32> {
        let mut map = HashMap::new();

        map.insert('I', 1);
        map.insert('V', 5);
        map.insert('X', 10);
        map.insert('L', 50);
        map.insert('C', 100);
        map.insert('D', 500);
        map.insert('M', 1000);

        map
    }

    pub fn roman_to_int(s: String) -> i32 {
        let map: HashMap<char, i32> = Solution::roman_to_int_map();
        let mut last: i32 = i32::MAX;

        s.chars().fold(0, |mut sum, c| {
            let curr = map[&c];
            if curr > last {
                sum -= last;
                sum += curr - last;
            } else {
                sum += curr;
            }

            last = map[&c];
            sum
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let s = input.read_as::<String>()?;

    let n = Solution::roman_to_int(s);
    println!("{}", n);

    Ok(())
}
