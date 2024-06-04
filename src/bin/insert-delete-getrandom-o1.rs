// insert-delete-getrandom-o1.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 380. Insert Delete GetRandom `O(1)`
//!
//!  - https://leetcode.com/problems/insert-delete-getrandom-o1

use rand::prelude::ThreadRng;
use rand::Rng;
use std::collections::HashSet;
use std::error::Error;

struct RandomizedSet {
    h: HashSet<i32>,
    rng: ThreadRng,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            h: HashSet::new(),
            rng: rand::thread_rng(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        self.h.insert(val)
    }

    fn remove(&mut self, val: i32) -> bool {
        self.h.remove(&val)
    }

    fn get_random(&mut self) -> i32 {
        if self.h.len() == 0 {
            panic!("no elements in the RandomizedSet");
        }

        let mut iter = self.h.iter();
        let mut pos = self.rng.gen_range(0..self.h.len());

        loop {
            if pos == 0 {
                break *iter.next().unwrap();
            }
            iter.next();
            pos -= 1;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut set = RandomizedSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);
    set.insert(5);
    set.insert(6);
    set.remove(6);
    println!("{}", set.get_random());

    Ok(())
}
