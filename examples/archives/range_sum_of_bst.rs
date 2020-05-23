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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        if let Some(n) = root {
            let mut b = n.borrow_mut();
            let mut v = 0;
            if l <= b.val && b.val <= r {
                v += b.val;
            }

            v += Self::range_sum_bst(b.left.take(), l, r);
            v += Self::range_sum_bst(b.right.take(), l, r);

            return v;
        }

        0
    }
}

fn main() {
    assert_eq!(Solution::range_sum_bst(None, 6, 10), 0);
}
