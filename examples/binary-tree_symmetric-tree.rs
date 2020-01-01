// #[derive(Debug, PartialEq, Eq)]
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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn _is_symmetric(
            node: Option<Rc<RefCell<TreeNode>>>,
            result: &mut Vec<Option<i32>>,
            reverse: bool,
        ) {
            match node {
                Some(n) => {
                    let n_b = n.borrow();
                    result.push(Some(n_b.val));

                    if reverse {
                        _is_symmetric(n_b.right.clone(), result, reverse);
                        _is_symmetric(n_b.left.clone(), result, reverse);
                    } else {
                        _is_symmetric(n_b.left.clone(), result, reverse);
                        _is_symmetric(n_b.right.clone(), result, reverse);
                    }
                }
                None => result.push(None),
            }
        }

        match root {
            None => true,
            Some(n) => {
                let mut left = Vec::new();
                let mut right = Vec::new();

                let n_b = n.borrow();

                _is_symmetric(n_b.left.clone(), &mut left, false);
                _is_symmetric(n_b.right.clone(), &mut right, true);

                left == right
            }
        }
    }
}

fn main() {
    let node = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: None,
        }))),
    })));

    println!("{}", Solution::is_symmetric(node));
}
