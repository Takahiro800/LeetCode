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
use std::rc::Rc;

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_between_val(root.as_ref(), None, None)
    }

    fn is_between_val(node: Option<&Node>, min: Option<i32>, max: Option<i32>) -> bool {
        if let Some(node) = node {
            let node = node.borrow();
            if min.map_or(false, |min_val| node.val <= min_val)
                || max.map_or(false, |max_val| node.val >= max_val)
            {
                return false;
            }

            Self::is_between_val(node.left.as_ref(), min, Some(node.val))
                && Self::is_between_val(node.right.as_ref(), Some(node.val), max)
        } else {
            true
        }
    }
}
