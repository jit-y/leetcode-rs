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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        fn _path_sum(
            node: Option<Rc<RefCell<TreeNode>>>,
            current: Vec<i32>,
            map: &mut std::collections::HashMap<i32, Vec<Vec<i32>>>,
        ) {
            if let Some(rc) = node {
                let b = rc.borrow();
                let mut current = current;
                current.push(b.val);

                if b.left.is_none() && b.right.is_none() {
                    let sum = current.iter().sum();
                    (*map.entry(sum).or_insert(Vec::new())).push(current);

                    return;
                }

                if b.left.is_some() {
                    _path_sum(b.left.clone(), current.clone(), map);
                }

                if b.right.is_some() {
                    _path_sum(b.right.clone(), current, map);
                }
            }
        }

        let mut map = std::collections::HashMap::new();
        let current = Vec::new();

        _path_sum(root, current, &mut map);

        for (v, arr) in map {
            if v == sum {
                return arr;
            }
        }

        vec![]
    }
}

fn main() {
    let d = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 13,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
    })));

    assert_eq!(
        Solution::path_sum(d, 22),
        vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
    )
}
