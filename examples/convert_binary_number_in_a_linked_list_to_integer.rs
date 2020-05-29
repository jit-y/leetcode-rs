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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut node = head;
        let mut result = 0;
        while let Some(mut l) = node {
            result = result << 1;
            result |= l.val;

            node = l.next.take();
        }

        result
    }
}

fn main() {
    fn create_list_from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut ret = None;
        for n in v.into_iter().rev() {
            let mut ln = ListNode::new(n);
            ln.next = ret;

            ret = Some(Box::new(ln));
        }

        ret
    }

    assert_eq!(
        Solution::get_decimal_value(create_list_from_vec(vec![1, 0, 1])),
        5
    );
    assert_eq!(
        Solution::get_decimal_value(create_list_from_vec(vec![0])),
        0
    );
    assert_eq!(
        Solution::get_decimal_value(create_list_from_vec(vec![1])),
        1
    );
    assert_eq!(
        Solution::get_decimal_value(create_list_from_vec(vec![
            1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0
        ])),
        18880
    );
    assert_eq!(
        Solution::get_decimal_value(create_list_from_vec(vec![0, 0])),
        0
    );
}
