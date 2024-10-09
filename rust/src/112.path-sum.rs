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

#[allow(dead_code)]
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::dfs(&root, target_sum)
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = node {
            let node = node.borrow();
            let rem = target_sum - node.val;

            if node.left.is_none() && node.right.is_none() {
                return rem == 0;
            }

            Self::dfs(&node.left, rem) || Self::dfs(&node.right, rem)
        } else {
            false
        }
    }
}
