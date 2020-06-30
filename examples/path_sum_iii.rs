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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let v = Self::_path_sum(root.clone(), sum);

        match root {
            None => v,
            Some(n) => {
                let mut b = n.borrow_mut();
                let left = Self::path_sum(b.left.take(), sum);
                let right = Self::path_sum(b.right.take(), sum);

                v + left + right
            }
        }
    }

    fn _path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        match root {
            None => 0,
            Some(n) => {
                let b = n.borrow();

                let mut c = 0;
                if b.val == sum {
                    c += 1;
                }

                c += Self::_path_sum(b.left.clone(), sum - b.val);
                c += Self::_path_sum(b.right.clone(), sum - b.val);

                c
            }
        }
    }
}

fn main() {
    assert_eq!(
        Solution::path_sum(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 3,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: -2,
                            left: None,
                            right: None
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: None
                        })))
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: -3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 11,
                        left: None,
                        right: None
                    })))
                }))),
            }))),
            8
        ),
        3
    );
}
