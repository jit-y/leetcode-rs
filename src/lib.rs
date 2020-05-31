#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut ret = None;
        for n in v.into_iter().rev() {
            let mut ln = ListNode::new(n);
            ln.next = ret;

            ret = Some(Box::new(ln));
        }

        ret
    }
}
