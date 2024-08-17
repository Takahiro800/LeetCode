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
        let mut cur = &mut head;

        while let Some(current) = cur {
            while let Some(next) = &mut current.next {
                if current.val == next.val {
                    current.next = next.next.take();
                } else {
                    break;
                }
            }

            cur = &mut current.next;
        }

        head
    }
}
