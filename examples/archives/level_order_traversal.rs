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
                queue.push_back(n);
            }
        }

        loop {
            if queue.is_empty() {
                break;
            }

            let mut queue_size = queue.len();
            let mut current_level = Vec::new();

            loop {
                if queue_size <= 0 {
                    break;
                }

                let node = match queue.pop_front() {
                    None => break,
                    Some(n) => n,
                };

                let inner = node.borrow();

                if let Some(n) = &inner.left {
                    queue.push_back((*n).clone());
                };

                if let Some(n) = &inner.right {
                    queue.push_back((*n).clone());
                };

                current_level.push(inner.val);

                queue_size -= 1;
            }

            if !current_level.is_empty() {
                result.push(current_level);
            }
        }

        result
    }
}

fn main() {}
