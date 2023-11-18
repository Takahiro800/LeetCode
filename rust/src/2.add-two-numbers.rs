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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let (mut l1, mut l2, mut carry, mut current) = (l1, l2, 0, &mut head);

        while l1.is_some() || l2.is_some() || carry > 0 {
            let sum = carry + l1.as_ref().map_or(0, |n| n.val) + l2.as_ref().map_or(0, |n| n.val);

            carry = sum / 10;

            if let Some(node) = current {
                node.next = Some(Box::new(ListNode::new(sum % 10)));
                current = &mut node.next;
            }
            l1 = l1.and_then(|n| n.next);
            l2 = l2.and_then(|n| n.next);
        }

        head.unwrap().next
    }
}
