// https://leetcode.com/problems/palindrome-linked-list/
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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut x = Vec::new();
        let mut y = Vec::new();

        let mut node = head;

        while let Some(v) = node {
            x.push(v.val);
            y.insert(0, v.val);

            node = v.next;
        }

        x == y
    }

    pub fn is_palindrome_2(head: Option<Box<ListNode>>) -> bool {
        let mut node = &head;
        let mut reverse = None;

        while let Some(v) = &node {
            let mut n = ListNode::new(v.val);
            n.next = reverse;

            reverse = Some(Box::new(n));
            node = &v.next;
        }

        head == reverse
    }

    pub fn is_palindrome_3(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }
        let mut node = head;
        let mut reverse = None;

        while let Some(v) = node.as_mut() {
            if reverse == v.next {
                return true;
            }

            let n = v.next.take();
            v.next = reverse;
            reverse = node;
            node = n;

            if reverse == node {
                return true;
            }
        }

        false
    }
}

fn main() {
    assert_eq!(
        Solution::is_palindrome(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None }))
        }))),
        false
    );
    assert_eq!(
        Solution::is_palindrome(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 1, next: None }))
                }))
            }))
        }))),
        true
    );
    assert_eq!(
        Solution::is_palindrome(Some(Box::new(ListNode {
            val: -129,
            next: Some(Box::new(ListNode {
                val: -129,
                next: None
            }))
        }))),
        true
    );
    assert_eq!(
        Solution::is_palindrome_2(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None }))
        }))),
        false
    );
    assert_eq!(
        Solution::is_palindrome_2(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 1, next: None }))
                }))
            }))
        }))),
        true
    );
    assert_eq!(
        Solution::is_palindrome_2(Some(Box::new(ListNode {
            val: -129,
            next: Some(Box::new(ListNode {
                val: -129,
                next: None
            }))
        }))),
        true
    );
}
