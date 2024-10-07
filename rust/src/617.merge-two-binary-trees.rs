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
    pub fn merge_trees(root1: Option<Node>, root2: Option<Node>) -> Option<Node> {
        Self::merge_trees_recursion(&root1, &root2)
    }

    fn merge_trees_recursion(node1: &Option<Node>, node2: &Option<Node>) -> Option<Node> {
        match (node1, node2) {
            (None, None) => None,
            (Some(node), None) | (None, Some(node)) => Some(node.clone()),
            (Some(node1), Some(node2)) => {
                let node1 = node1.borrow();
                let node2 = node2.borrow();

                let mut root = TreeNode::new(node1.val + node2.val);
                root.left = Self::merge_trees(node1.left.clone(), node2.left.clone());
                root.right = Self::merge_trees(node1.right.clone(), node2.right.clone());

                Some(Rc::new(RefCell::new(root)))
            }
        }
    }
}
