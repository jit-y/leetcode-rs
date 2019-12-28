use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
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

struct Solution {}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();

        match root {
            None => {
                return result;
            }
            Some(node) => {
                stack.push(node);
            }
        }

        loop {
            let node = match stack.pop() {
                Some(n) => n,
                None => break,
            };

            let inner = node.borrow();

            match &inner.left {
                None => {
                    result.push(inner.val);
                }
                Some(left) => {
                    let mut new_node = TreeNode::new(inner.val);
                    new_node.right = inner.right.clone();
                    stack.push(Rc::new(RefCell::new(new_node)));
                    stack.push(left.clone());
                    continue;
                }
            }

            match &inner.right {
                None => {}
                Some(right) => {
                    let mut new_node = TreeNode::new(inner.val);
                    stack.push(right.clone());
                }
            }
        }

        result
    }
}

fn main() {}
