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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut result = vec![];

        queue.push_back(root);

        while !queue.is_empty() {
            let mut count = 0;
            let mut val = 0f64;
            let mut next_queue = VecDeque::new();

            while let Some(opt) = queue.pop_front() {
                if let Some(node) = opt {
                    let mut b = node.borrow_mut();

                    next_queue.push_back(b.left.take());
                    next_queue.push_back(b.right.take());

                    count += 1;
                    val += b.val as f64;
                }
            }

            queue = next_queue;
            if count > 0 {
                result.push(val / count as f64);
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::average_of_levels(Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None
                }))),
            }))),
        })))),
        vec![3.0, 14.5, 11.0]
    );
}
