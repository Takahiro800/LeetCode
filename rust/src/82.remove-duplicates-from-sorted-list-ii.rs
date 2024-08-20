use crate::{remove_duplicates_from_sorted_list::ListNode, Solution};

impl Solution {
    // function nameが被っているので_iiをつけている。提出する際は消すこと
    pub fn delete_duplicates_ii(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut prev = new_head.as_mut();

        while let Some(mut target) = prev.as_mut().unwrap().next.take() {
            match target.next.as_mut() {
                Some(next_node) => {
                    if target.val == next_node.val {
                        let duplicate_node = Self::skip_duplicates(target);
                        prev.as_mut().unwrap().next = duplicate_node.next;
                    } else {
                        prev.as_mut().unwrap().next = Some(target);
                        prev = prev.unwrap().next.as_mut();
                    }
                }
                None => {
                    prev.as_mut().unwrap().next = Some(target);
                    break;
                }
            }
        }

        new_head.unwrap().next
    }

    fn skip_duplicates(mut node: Box<ListNode>) -> Box<ListNode> {
        while let Some(next_node) = node.next.as_mut() {
            if next_node.val != node.val {
                break;
            }

            node = node.next.take().unwrap();
        }
        node
    }
}
