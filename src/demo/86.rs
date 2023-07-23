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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        if (head.is_none() || head.as_ref().unwrap().next.is_none()) {
            return head;
        }
        let mut head = head;
        let (mut p1, mut p2) = (ListNode::new(0), ListNode::new(0));
        let mut p1_cur = &mut p1;
        let mut p2_cur = &mut p2;

        while let Some(mut node) = head {
            head = node.next.take();

            if node.val < x {
                p1_cur.next = Some(node);
                p1_cur = p1_cur.next.as_mut().unwrap();
            } else {
                p2_cur.next = Some(node);
                p2_cur = p2_cur.next.as_mut().unwrap();
            }
        }
        p1_cur.next = p2.next.take();
        p1.next
    }
}
