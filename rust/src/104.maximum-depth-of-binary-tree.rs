// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(root) => {
                // takeだとroot.left, root.rightがNoneになる
                // let left = Solution::max_depth(root.borrow_mut().left.take());
                // let right = Solution::max_depth(root.borrow_mut().right.take());
                let left = Solution::max_depth(root.borrow_mut().left.clone());
                let right = Solution::max_depth(root.borrow_mut().right.clone());
                1 + std::cmp::max(left, right)
            }
            None => 0,
        }
    }
}
