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
struct Solution {}
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        let mut stack = Vec::new();

        match root {
            None => return result,
            Some(rc) => {
                stack.push(rc);
            }
        }

        while !stack.is_empty() {
            let rc = stack.pop().expect("pop");
            let b = rc.borrow();

            if let Some(rc) = &b.right {
                let b = rc.borrow();
                if b.left.is_some() || b.right.is_some() {
                    stack.push(rc.clone());
                }
            }

            if let Some(rc) = &b.left {
                let b = rc.borrow();

                if b.left.is_none() && b.right.is_none() {
                    result += b.val;
                } else {
                    stack.push(rc.clone());
                }
            }
        }

        result
    }
}

fn main() {
    let a = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));

    assert_eq!(Solution::sum_of_left_leaves(a), 24);
}
