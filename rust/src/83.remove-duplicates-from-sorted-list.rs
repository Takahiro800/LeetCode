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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = &mut head;

        while let Some(current) = cur.as_mut() {
            while let Some(next) = &mut current.next {
                if current.val != next.val {
                    break;
                }

                current.next = next.next.take();
            }

            cur = &mut current.next;
        }

        head
    }
}
