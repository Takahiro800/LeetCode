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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        recursive(head, None)
    }
}

fn recursive(head: Option<Box<ListNode>>, prev: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        None => prev,
        Some(mut node) => {
            let next_node = node.next.take();
            node.next = prev;
            recursive(next_node, Some(node))
        }
    }
}
