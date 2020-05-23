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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
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

            match &inner.right {
                None => {}
                Some(right) => {
                    stack.push(right.clone());
                }
            }

            match &inner.left {
                None => {}
                Some(left) => {
                    stack.push(left.clone());
                }
            }

            result.push(inner.val);
        }

        result
    }
}

fn main() {}
