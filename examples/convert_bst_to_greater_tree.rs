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
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn _convert_bst(root: &mut Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
            if let Some(node) = root {
                let mut b = node.borrow_mut();

                _convert_bst(&mut b.right, sum);

                let v = b.val;
                b.val += *sum;
                *sum += v;

                _convert_bst(&mut b.left, sum);
            }
        }

        let mut root = root;
        let mut sum = 0;

        _convert_bst(&mut root, &mut sum);

        root
    }
}
fn main() {
    assert_eq!(
        Solution::convert_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                right: None,
                left: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 13,
                right: None,
                left: None,
            }))),
        })))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 18,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                right: None,
                left: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 13,
                right: None,
                left: None,
            }))),
        })))
    );
}
