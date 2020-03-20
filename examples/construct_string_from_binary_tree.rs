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
    pub fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String {
        match t {
            None => "".to_string(),
            Some(n) => {
                let b = n.borrow();
                let mut result = b.val.to_string();

                if b.left.is_none() && b.right.is_none() {
                    return result;
                }

                result = format!("{}({})", result, Solution::tree2str(b.left.clone()));
                if b.right.is_some() {
                    result = format!("{}({})", result, Solution::tree2str(b.right.clone()));
                }

                result
            }
        }
    }
}

fn main() {
    assert_eq!(
        Solution::tree2str(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            }))),
        })))),
        "1(2(4))(3)".to_string(),
    );
}
