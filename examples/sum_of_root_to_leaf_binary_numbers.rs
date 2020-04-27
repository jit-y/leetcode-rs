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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, current: i32, result: &mut Vec<i32>) {
            if let Some(n) = node {
                let mut b = n.borrow_mut();

                let mut current = current;
                current = current << 1;
                current |= b.val;

                if b.left.is_none() && b.right.is_none() {
                    result.push(current);
                    return;
                }

                dfs(b.left.take(), current, result);
                dfs(b.right.take(), current, result);
            }
        }

        let mut arr = vec![];

        dfs(root, 0, &mut arr);

        arr.into_iter().sum()
    }
}

fn main() {
    assert_eq!(
        Solution::sum_root_to_leaf(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None
                }))),
            }))),
        })))),
        22
    );

    assert_eq!(Solution::sum_root_to_leaf(None), 0);
    assert_eq!(
        Solution::sum_root_to_leaf(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None
        })))),
        1
    );
}
