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
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut queue = std::collections::VecDeque::new();

        match root {
            None => {
                return result;
            }
            Some(n) => {
                let mut current_level = Vec::new();

                {
                    let inner = n.borrow();
                    current_level.push(inner.val);
                }

                result.push(current_level);

                queue.push_back(n.clone());
            }
        }

        loop {
            let node = match queue.pop_front() {
                None => break,
                Some(n) => n,
            };

            let inner = node.borrow();
            let mut current_level = Vec::new();

            if let Some(n) = &inner.left {
                {
                    let borrowed = n.borrow();
                    current_level.push(borrowed.val);
                }

                queue.push_back(n.clone());
            };

            if let Some(n) = &inner.right {
                {
                    let borrowed = n.borrow();
                    current_level.push(borrowed.val);
                }

                queue.push_back(n.clone());
            };

            if !current_level.is_empty() {
                result.push(current_level);
            }
        }

        result
    }
}
