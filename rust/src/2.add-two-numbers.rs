use crate::Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let (mut current, mut carry, mut l1, mut l2) = (&mut head, 0_i32, l1, l2);

        while l1.is_some() || l2.is_some() || carry.is_positive() {
            let sum = carry
                + l1.as_ref().map_or(0, |node| node.val)
                + l2.as_ref().map_or(0, |node| node.val);
            carry = sum / 10;

            if let Some(node) = current {
                node.next = Some(Box::new(ListNode::new(sum % 10)));
                current = &mut node.next;
            }
            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        head.and_then(|head| head.next)
    }
}
