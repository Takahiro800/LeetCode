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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut beginning = &mut dummy;

        // while let (Some(n1), Some(n2)) = (Some(l1), Some(l2)) {
        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
                let next = list1.as_mut().unwrap().next.take();
                beginning.next = list1.take();
                list1 = next;
            } else {
                let next = list2.as_mut().unwrap().next.take();
                beginning.next = list2.take();
                list2 = next;
            }
            beginning = beginning.next.as_mut().unwrap();
        }
        if list1.is_some() {
            beginning.next = list1;
        } else {
            beginning.next = list2;
        }
        dummy.next
    }
}
