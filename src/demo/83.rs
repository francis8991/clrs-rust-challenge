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
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if (head.is_none()) {
            return head;
        }

        let mut slow = head.as_mut().unwrap();

        while let Some(node) = slow.next.take() {
            if node.val != slow.val {
                slow.next = Some(node);
                slow = slow.next.as_mut().unwrap();
            } else {
                slow.next = node.next;
            }
        }
        slow.next = None;
        head
    }
}
