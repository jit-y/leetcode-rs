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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn gather(node: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
            if let Some(rc) = node {
                let mut b = rc.borrow_mut();

                if b.left.is_some() || b.right.is_some() {
                    gather(b.left.take(), arr);
                    gather(b.right.take(), arr);

                    return;
                }

                arr.push(b.val);
            }
        }

        let mut r1 = Vec::new();
        let mut r2 = Vec::new();

        gather(root1, &mut r1);
        gather(root2, &mut r2);

        r1 == r2
    }
}

fn main() {
    assert_eq!(Solution::leaf_similar(None, None), true);
}
