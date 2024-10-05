use crate::Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

#[allow(dead_code)]
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::min_depth_recursion(root.as_ref())
    }

    fn min_depth_recursion(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => match (node.borrow().left.as_ref(), node.borrow().right.as_ref()) {
                (None, None) => 1,
                (Some(left), None) => Self::min_depth_recursion(Some(left)) + 1,
                (None, Some(right)) => Self::min_depth_recursion(Some(right)) + 1,
                (Some(left), Some(right)) => {
                    cmp::min(
                        Self::min_depth_recursion(Some(left)),
                        Self::min_depth_recursion(Some(right)),
                    ) + 1
                }
            },
            None => 0,
        }
    }
}
