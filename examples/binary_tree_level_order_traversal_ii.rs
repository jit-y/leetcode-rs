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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut result = vec![];

        while !queue.is_empty() {
            let mut arr = vec![];
            let mut next_queue = VecDeque::new();

            while let Some(opt) = queue.pop_front() {
                if let Some(node) = opt {
                    let mut b = node.borrow_mut();
                    arr.push(b.val);

                    next_queue.push_back(b.left.take());
                    next_queue.push_back(b.right.take());
                }
            }

            if !arr.is_empty() {
                result.insert(0, arr);
            }
            queue = next_queue;
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::level_order_bottom(Some(Rc::new(RefCell::new(TreeNode {
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
        vec![vec![15, 7], vec![9, 20], vec![3]]
    );
}
