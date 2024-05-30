// remove-nodes-from-linked-list.rs, My solutions to various Leetcode puzzles.
// Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

#![allow(dead_code, unused_mut)]

//! # 2487. Remove Nodes From Linked List
//!
//!  - https://leetcode.com/problems/remove-nodes-from-linked-list

use leetcode::*;
use std::error::Error;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    /// You are given the `head` of a linked list.
    ///
    /// Remove every node which has a node with a greater value anywhere to the
    /// right side of it.
    ///
    /// Return the `head` of the modified linked list.
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head;
        let mut stack: Vec<Box<ListNode>> = Vec::new();

        while current.is_some() {
            let next = current.as_mut().and_then(|x| x.next.take());
            stack.push(current.unwrap());
            current = next;
        }

        let mut max = i32::MIN;
        current = None;

        while !stack.is_empty() {
            let mut node = stack.pop()?;
            let val = node.val;
            if val >= max {
                max = node.val;
                node.next = current;
                current = Some(node);
            }
        }

        current
    }
}

#[cfg(not(feature = "count-allocations"))]
fn run(head: Box<ListNode>) {
    let mut answer = Solution::remove_nodes(head.next);
    loop {
        match answer {
            Some(curr) => {
                print!("{} ", curr.val);
                answer = curr.next;
            }
            None => break,
        }
    }
    println!();
}

#[cfg(feature = "count-allocations")]
fn run(head: Box<ListNode>) {
    let info = allocation_counter::measure(|| {
        let mut answer = Solution::remove_nodes(head.next);
    });
    println!("allocation count = {}", info.count_total);
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = std::io::stdin().lock();

    let Vector(v) = input.read_line_as::<Vector<i32>>()?;

    let mut head = Box::new(ListNode::new(0));

    for elem in v.iter().rev() {
        head.val = *elem;

        let mut node = Box::new(ListNode::new(0));
        node.next = Some(head);
        head = node;
    }

    run(head);

    Ok(())
}
