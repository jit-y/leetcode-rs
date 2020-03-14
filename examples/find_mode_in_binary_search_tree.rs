// https://leetcode.com/problems/find-mode-in-binary-search-tree/
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
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn store_value(
            node: Option<Rc<RefCell<TreeNode>>>,
            map: &mut std::collections::HashMap<i32, i32>,
            max: i32,
        ) -> i32 {
            let mut current = max;

            if let Some(n) = node {
                let b = n.borrow();
                let l = store_value(b.left.clone(), map, current);
                let r = store_value(b.right.clone(), map, current);

                let ent = map.entry(b.val).or_insert(0);
                *ent += 1;
                if *ent > current {
                    current = *ent;
                }

                if l > current {
                    current = l;
                }

                if r > current {
                    current = r;
                }
            }

            current
        }

        let mut map = std::collections::HashMap::new();
        let max = store_value(root, &mut map, 0);

        let mut result = Vec::new();
        for (k, v) in map {
            if v == max {
                result.push(k);
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::find_mode(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None
                }))),
                right: None
            })))
        })))),
        vec![2]
    );
}
