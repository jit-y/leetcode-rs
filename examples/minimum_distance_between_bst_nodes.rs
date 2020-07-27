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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::cmp;
        fn collect(root: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
            if let Some(n) = root {
                let mut b = n.borrow_mut();

                arr.push(b.val);

                collect(b.left.take(), arr);
                collect(b.right.take(), arr);
            }
        }

        let mut arr = vec![];
        collect(root, &mut arr);
        arr.sort();

        let mut min = std::i32::MAX;

        for i in 1..arr.len() {
            min = cmp::min(min, (arr[i - 1] - arr[i]).abs());
        }

        min
    }
}

fn main() {
    assert_eq!(
        Solution::min_diff_in_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
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
                val: 6,
                left: None,
                right: None
            }))),
        })))),
        1
    )
}
