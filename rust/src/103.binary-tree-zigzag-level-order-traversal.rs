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

type Node = Rc<RefCell<TreeNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let depth = Self::max_depth_p(root.as_ref());
        let mut ans = vec![vec![]; depth];

        Self::level_order_recursive(root.as_ref(), &mut ans, 0);
        Self::zigzag(&mut ans);

        ans
    }

    fn level_order_recursive(node: Option<&Node>, array: &mut Vec<Vec<i32>>, current_depth: usize) {
        if let Some(node) = node {
            array[current_depth].push(node.borrow().val);

            Self::level_order_recursive(node.borrow().left.as_ref(), array, current_depth + 1);
            Self::level_order_recursive(node.borrow().right.as_ref(), array, current_depth + 1);
        }
    }

    fn zigzag(array: &mut Vec<Vec<i32>>) {
        for (i, arr) in array.iter_mut().enumerate() {
            if i % 2 == 1 {
                arr.reverse()
            }
        }
    }

    fn max_depth_p(node: Option<&Node>) -> usize {
        Self::depth_recursive(node)
    }

    fn depth_recursive(node: Option<&Node>) -> usize {
        match node {
            Some(node) => {
                cmp::max(
                    Self::depth_recursive(node.borrow().left.as_ref()),
                    Self::depth_recursive(node.borrow().right.as_ref()),
                ) + 1
            }
            None => 0,
        }
    }
}
