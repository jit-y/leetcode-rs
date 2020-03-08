// https://leetcode.com/problems/remove-linked-list-elements/
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        match head {
            Some(n) => {
                let nx = Self::remove_elements(n.next, val);
                if n.val == val {
                    nx
                } else {
                    let new = ListNode {
                        val: n.val,
                        next: nx,
                    };
                    Some(Box::new(new))
                }
            }
            None => {
                return None;
            }
        }
    }
}

fn main() {
    assert_eq!(
        Solution::remove_elements(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode { val: 6, next: None }))
                                }))
                            }))
                        }))
                    }))
                }))
            })),
            6
        ),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None }))
                    }))
                }))
            }))
        })),
    );
}
