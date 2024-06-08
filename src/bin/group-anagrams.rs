// group-anagrams.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 49. Group Anagrams
//!
//!  - https://leetcode.com/problems/group-anagrams

use leetcode::*;
use std::collections::HashMap;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn group_anagrams(strings: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();

        let string_hash = |string: &str| {
            let mut chars = string.chars().collect::<Vec<_>>();
            chars.sort();
            String::from_iter(chars)
        };

        for string in strings.into_iter() {
            let key = string_hash(&string);
            let val = groups.get_mut(&key);

            if let Some(group) = val {
                group.push(string);
            } else {
                let mut group: Vec<String> = Vec::new();
                group.push(string);
                groups.insert(key, group);
            }
        }

        groups.into_iter().fold(vec![], |mut result, (_, val)| {
            result.push(val);
            result
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let Vector(strings) = input.read_line_as::<Vector<String>>()?;

    let groups = Solution::group_anagrams(strings);
    println!("{groups:?}");

    Ok(())
}
