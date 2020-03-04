// https://leetcode.com/problems/remove-duplicates-from-sorted-list/
// Definition for singly-linked list.
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = head;
        let mut node = &mut root;

        while let Some(n) = node {
            let v = n.val;

            while let Some(nx) = &mut n.next {
                if v != nx.val {
                    break;
                }

                let a = nx.next.take();
                n.next = a;
            }

            node = &mut n.next;
        }

        root
    }
}

fn main() {
    assert_eq!(
        Solution::delete_duplicates(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None }))
            }))
        }))),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None }))
        }))
    );
    assert_eq!(
        Solution::delete_duplicates(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 1, next: None }))
            }))
        }))),
        Some(Box::new(ListNode { val: 1, next: None }))
    );
}
