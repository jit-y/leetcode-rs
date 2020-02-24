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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut root = Box::new(ListNode::new(0));
        let mut current = &mut root;
        let mut l1_current = l1;
        let mut l2_current = l2;

        while l1_current.is_some() && l2_current.is_some() {
            let l1_some = l1_current.take();
            let l2_some = l2_current.take();

            if let (Some(mut n1), Some(mut n2)) = (l1_some, l2_some) {
                if n1.val <= n2.val {
                    l1_current = n1.next.take();
                    l2_current = Some(n2);
                    current = current.next.get_or_insert(n1);
                } else {
                    l1_current = Some(n1);
                    l2_current = n2.next.take();
                    current = current.next.get_or_insert(n2);
                }
            }
        }

        if l1_current.is_some() {
            current.next = l1_current;
        }

        if l2_current.is_some() {
            current.next = l2_current;
        }

        root.next
    }
}

fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let result = Solution::merge_two_lists(l1, l2);

    println!("{:?}", result);
}
