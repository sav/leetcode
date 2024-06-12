// substring-with-concatenation-of-all-words.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 30. Substring With Concatenation Of All Words
//!
//!  - https://leetcode.com/problems/substring-with-concatenation-of-all-words

use leetcode::*;
use std::collections::HashMap;
use std::error::Error;

struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let numb_of_words = words.len();

        if numb_of_words < 1 {
            return vec![];
        }

        let word_len = words[0].len();
        let s_len = s.len();

        if s_len < numb_of_words * word_len {
            return vec![];
        }

        // Map each (sorted) word to its count. The count is necessary to
        // account for repeated words.
        let mut word_count_map: HashMap<&str, i32> = HashMap::with_capacity(numb_of_words);
        for word in &words {
            *word_count_map.entry(word).or_insert(0) += 1;
        }

        // Slide the window through the input string, looking for concatenated
        // strings. We will need separate maps for the slinding windows to keep
        // track of the current word counts.
        let mut indexes: Vec<i32> = Vec::with_capacity(s_len);

        for start in 0..word_len {
            let mut window_count_map: HashMap<&str, i32> = HashMap::with_capacity(numb_of_words);
            let mut words_found = 0;
            let mut left = start;

            for right in (start..=s_len - word_len).step_by(word_len) {
                let word = &s[right..right + word_len];

                if let Some(&word_count) = word_count_map.get(word) {
                    words_found += 1;
                    *window_count_map.entry(word).or_insert(0) += 1;

                    // Try and shrink the window if exceeded the count for this particular `word`.
                    while window_count_map[word] > word_count {
                        let first = &s[left..left + word_len];
                        *window_count_map.get_mut(first).unwrap() -= 1;
                        left += word_len;
                        words_found -= 1;
                    }

                    if words_found == numb_of_words {
                        indexes.push(left as i32);
                    }
                } else {
                    // Sequence broken: restart and shift right.
                    left = right + word_len;
                    words_found = 0;
                    window_count_map.clear();
                }
            }
        }

        indexes
    }
}

#[cfg(not(feature = "count-allocations"))]
#[inline]
fn run(s: String, words: Vec<String>) {
    let answer = Solution::find_substring(s, words);
    println!("{answer:?}");
}

#[cfg(feature = "count-allocations")]
#[inline]
fn run(s: String, words: Vec<String>) {
    let info = allocation_counter::measure(|| {
        let answer = Solution::find_substring(s, words);
        println!("{answer:?}");
    });
    println!("allocation count = {}", info.count_total);
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let s = input.read_line_as::<String>()?;
    let Vector(words) = input.read_line_as::<Vector<String>>()?;

    run(s, words);

    Ok(())
}
