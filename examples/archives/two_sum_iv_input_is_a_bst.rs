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
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);
        let mut map = std::collections::HashMap::new();

        while !queue.is_empty() {
            let node = queue.pop_front().expect("pop_front");
            if let Some(n) = node {
                let mut b = n.borrow_mut();
                let gap = k - b.val;
                if let Some(_) = map.get(&gap) {
                    return true;
                }

                map.insert(b.val, true);
                queue.push_back(b.left.take());
                queue.push_back(b.right.take());
            }
        }

        false
    }
}

fn main() {
    assert_eq!(
        Solution::find_target(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None
                    }))),
                }))),
            }))),
            9
        ),
        true
    )
}
