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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head.clone();
        let mut slow = head;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = slow.as_mut().unwrap().next.take();
            fast = fast.as_mut().unwrap().next.as_mut().unwrap().next.take();
        }

        slow
    }
}

fn main() {
    assert_eq!(
        Solution::middle_node(Some(Box::new(ListNode {
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
        }))),
        Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 5, next: None }))
            }))
        }))
    )
}
