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
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32, total: i32, result: &mut bool) {
            if let Some(r) = root {
                let node = r.borrow();

                if node.left.is_none() && node.right.is_none() {
                    if sum == (total + node.val) {
                        *result = true
                    }
                }

                path_sum(node.left.clone(), sum, total + node.val, result);
                path_sum(node.right.clone(), sum, total + node.val, result);
            }
        }

        match root {
            None => false,
            Some(n) => {
                let node = n.borrow();

                if node.left.is_none() && node.right.is_none() {
                    return node.val == sum;
                }

                let mut result = false;

                if node.left.is_some() {
                    path_sum(node.left.clone(), sum, node.val, &mut result);
                }

                if node.right.is_some() {
                    path_sum(node.right.clone(), sum, node.val, &mut result);
                }

                result
            }
        }
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
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    let b = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: None,
    })));

    println!("{}", Solution::has_path_sum(a, 30));
    println!("{}", Solution::has_path_sum(b, 3));
}
