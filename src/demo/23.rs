// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use std::{
    cmp::{Ord, Ordering, PartialEq},
    collections::BinaryHeap,
};

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        let mut heap = BinaryHeap::new();
        let mut l = ListNode::new(-1);
        let mut l_pr = &mut l;

        for list in lists {
            if let Some(node) = list {
                heap.push(node);
            }
        }

        while let Some(mut node) = heap.pop() {
            if let Some(n) = node.next.take() {
                heap.push(n);
            }
            l_pr.next = Some(node);
            l_pr = l_pr.next.as_mut().unwrap();
        }
        l.next
    }
}
