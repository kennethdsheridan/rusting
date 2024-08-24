#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
}

struct Solution {}

type List = Option<Box<ListNode>>;


use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn merge_lists(lists: Vec<List>) -> List {

        // create a min-heap
        let mut heap = BinaryHeap::new(); 
    }
}

fn main() {}
