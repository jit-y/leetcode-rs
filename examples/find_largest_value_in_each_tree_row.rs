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
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut result = vec![];
        if root.is_some() {
            queue.push_back(root);
        }

        while !queue.is_empty() {
            let mut next = VecDeque::new();
            let mut max_val = std::i32::MIN;

            while let Some(op) = queue.pop_front() {
                let n = op.expect("oops");
                let mut b = n.borrow_mut();

                max_val = std::cmp::max(max_val, b.val);
                if b.left.is_some() {
                    next.push_back(b.left.take());
                }

                if b.right.is_some() {
                    next.push_back(b.right.take());
                }
            }

            queue = next;
            result.push(max_val);
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::largest_values(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None
                })))
            }))),
        })))),
        vec![1, 3, 9]
    );
    assert_eq!(Solution::largest_values(None), vec![]);
}
