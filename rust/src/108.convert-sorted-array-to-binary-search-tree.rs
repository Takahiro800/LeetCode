use crate::Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        };

        let mid = nums.len() / 2;

        let mut node = TreeNode::new(nums[mid]);
        node.left = Self::sorted_array_to_bst(nums[..mid].to_vec());
        node.right = Self::sorted_array_to_bst(nums[mid + 1..].to_vec());

        Some(Rc::new(RefCell::new(node)))
    }
}
