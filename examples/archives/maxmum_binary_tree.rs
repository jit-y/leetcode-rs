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
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn construct(nums: &[i32], left: usize, right: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if left == right {
                return None;
            }

            let mut max_idx = left;
            for i in (left + 1)..right {
                if nums[max_idx] < nums[i] {
                    max_idx = i;
                }
            }

            let mut t = TreeNode::new(nums[max_idx] as i32);
            t.left = construct(nums, left, max_idx);
            t.right = construct(nums, max_idx + 1, right);

            Some(Rc::new(RefCell::new(t)))
        }

        if nums.is_empty() {
            return None;
        }

        construct(&nums, 0, nums.len())
    }
}

fn main() {
    assert_eq!(
        Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None
                    })))
                })))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
        })))
    );
}
