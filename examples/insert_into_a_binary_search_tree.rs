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
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn insert_into(root: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
            match root {
                Some(n) => {
                    let mut b = n.borrow_mut();
                    if val < b.val {
                        if b.left.is_none() {
                            b.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                        } else {
                            insert_into(&mut b.left, val);
                        }
                    } else {
                        if b.right.is_none() {
                            b.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                        } else {
                            insert_into(&mut b.right, val);
                        }
                    }
                }
                None => *root = Some(Rc::new(RefCell::new(TreeNode::new(val)))),
            }
        }

        let mut root = root;

        insert_into(&mut root, val);

        root
    }
}

fn main() {
    assert_eq!(
        Solution::insert_into_bst(None, 5),
        Some(Rc::new(RefCell::new(TreeNode::new(5))))
    );
    assert_eq!(
        Solution::insert_into_bst(
            Some(Rc::new(RefCell::new(TreeNode {
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
                    val: 7,
                    left: None,
                    right: None
                }))),
            }))),
            5
        ),
        Some(Rc::new(RefCell::new(TreeNode {
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
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
        })))
    );
}
