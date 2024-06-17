// merge-sorted-array.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

//! # 88. Merge Sorted Array
//!
//!  - https://leetcode.com/problems/merge-sorted-array

use leetcode::*;
use std::error::Error;

struct Solution;

impl Solution {
    /// You are given two integer arrays `nums1` and `nums2`, sorted in
    /// *non-decreasing order*, and two integers `m` and `n`, representing the
    /// number of elements in `nums1` and `nums2` respectively.
    ///
    /// Merge `nums1` and `nums2` into a single array sorted in non-decreasing
    /// order.
    ///
    /// The final sorted array should not be returned by the function, but
    /// instead be stored inside the array `nums1`.
    ///
    /// To accommodate this, `nums1` has a length of `m + n`, where the first
    /// `m` elements denote the elements that should be merged, and the last `n`
    /// elements are set to `0` and should be ignored. `nums2` has a length of
    /// `n`.
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut m, mut n) = (m as usize, n as usize);

        for pos in (0..m + n).rev() {
            assert_eq!(m > 0 || n > 0, true);

            nums1[pos] = if n == 0 {
                m -= 1;
                nums1[m]
            } else if m == 0 {
                n -= 1;
                nums2[n]
            } else if nums1[m - 1] > nums2[n - 1] {
                m -= 1;
                nums1[m]
            } else {
                n -= 1;
                nums2[n]
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let Vector(mut nums1) = input.read_as::<Vector<i32>>()?;
    let m: i32 = input.read_as()?;

    let Vector(mut nums2) = input.read_as::<Vector<i32>>()?;
    let n: i32 = input.read_as()?;

    Solution::merge(&mut nums1, m, &mut nums2, n);

    println!("{:?}", nums1);

    Ok(())
}
