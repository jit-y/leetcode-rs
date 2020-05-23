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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        fn bt(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<String>, s: String) {
            if let Some(n) = node {
                let b = n.borrow();
                let s = if s.is_empty() {
                    format!("{}", b.val)
                } else {
                    format!("{}->{}", s, b.val)
                };
                if b.left.is_none() && b.right.is_none() {
                    result.push(s);
                    return;
                }

                bt(b.left.clone(), result, s.clone());
                bt(b.right.clone(), result, s);
            }
        }

        let mut result = Vec::new();

        bt(root, &mut result, String::new());

        result
    }
}

fn main() {
    assert_eq!(
        Solution::binary_tree_paths(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            })))
        })))),
        vec!["1->2->5".to_string(), "1->3".to_string()]
    );
    assert_eq!(
        Solution::binary_tree_paths(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None
        })))),
        vec!["1".to_string()]
    );
}
