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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let l = nums.len();
        let i = l / 2;

        let mut node = TreeNode::new(nums[i]);
        node.left = Self::sorted_array_to_bst((&nums[0..i]).to_vec());
        node.right = Self::sorted_array_to_bst((&nums[(i + 1)..l]).to_vec());

        Some(Rc::new(RefCell::new(node)))
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9])
    );
    println!("{:?}", Solution::sorted_array_to_bst(vec![]));
    println!("{:?}", Solution::sorted_array_to_bst(vec![1]));
    println!(
        "{:?}",
        Solution::sorted_array_to_bst(vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12])
    );
    println!("{:?}", Solution::sorted_array_to_bst(vec![1, 2, 3, 4]));
}
