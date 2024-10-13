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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let depth = Self::depth(root.as_ref());
        let mut ans = vec![vec![]; depth];

        Self::level_order_recurion(root.as_ref(), &mut ans, 0);

        ans
    }

    fn level_order_recurion(
        node: Option<&Rc<RefCell<TreeNode>>>,
        array: &mut Vec<Vec<i32>>,
        current_depth: usize,
    ) {
        if let Some(node) = node {
            array[current_depth].push(node.borrow().val);

            let next_depth = current_depth + 1;
            Self::level_order_recurion(node.borrow().left.as_ref(), array, next_depth);
            Self::level_order_recurion(node.borrow().right.as_ref(), array, next_depth);
        }
    }

    fn depth(node: Option<&Rc<RefCell<TreeNode>>>) -> usize {
        Self::depth_recursion(node)
    }

    fn depth_recursion(node: Option<&Rc<RefCell<TreeNode>>>) -> usize {
        match node {
            Some(node) => {
                cmp::max(
                    Self::depth_recursion(node.borrow().left.as_ref()),
                    Self::depth_recursion(node.borrow().right.as_ref()),
                ) + 1
            }
            None => 0,
        }
    }
}
