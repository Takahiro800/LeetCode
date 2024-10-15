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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut preorder = preorder;
        Self::build_tree_recursive(&mut preorder, &inorder)
    }

    fn build_tree_recursive(preorder: &mut Vec<i32>, inorder: &[i32]) -> Option<Node> {
        if inorder.is_empty() {
            return None;
        }

        let root_val = preorder.remove(0);
        let root_index = inorder.iter().position(|&x| x == root_val).unwrap();
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));

        root.borrow_mut().left = Self::build_tree_recursive(preorder, &inorder[..root_index]);
        root.borrow_mut().right = Self::build_tree_recursive(preorder, &inorder[root_index + 1..]);

        Some(root)
    }
}
