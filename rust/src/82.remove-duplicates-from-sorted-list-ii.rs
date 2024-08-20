use crate::Solution;

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

impl Solution {
    // function nameが被っているので_iiをつけている。提出する際は消すこと
    pub fn delete_duplicates_ii(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut prev = new_head.as_mut();

        while let Some(mut current_node) = prev.as_mut().unwrap().next.take() {
            if let Some(next_node) = current_node.next.as_mut() {
                if current_node.val == next_node.val {
                    let duplicate_node = Self::skip_duplicates(current_node);
                    prev.as_mut().unwrap().next = duplicate_node.next;
                } else {
                    prev.as_mut().unwrap().next = Some(current_node);
                    prev = prev.unwrap().next.as_mut();
                }
            } else {
                prev.as_mut().unwrap().next = Some(current_node);
                break;
            }
        }

        new_head.unwrap().next
    }

    fn skip_duplicates(mut node: Box<ListNode>) -> Box<ListNode> {
        while let Some(next_node) = node.next.as_mut() {
            if next_node.val == node.val {
                node = node.next.take().unwrap();
            } else {
                break;
            }
        }
        node
    }
}
