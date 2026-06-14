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
impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut fast_head = &head;
        let mut flipper_head = &head;
        while let Some(flipper_node) = flipper_head
            && let Some(fast_node) = fast_head
        {
            flipper_head = &flipper_node.next;
            fast_head = &fast_node.next.as_ref().unwrap().next;
        }

        let mut flipper_head = flipper_head.clone();
        let mut prev_node = None;
        while let Some(mut node) = flipper_head {
            flipper_head = node.next;
            node.next = prev_node;
            prev_node = Some(node);
        }

        let mut head_head = &head;
        let mut rev_head = &prev_node;
        let mut max = 0;
        while let Some(head_node) = &head_head
            && let Some(rev_node) = &rev_head
        {
            max = max.max(head_node.val + rev_node.val);
            head_head = &head_node.next;
            rev_head = &rev_node.next;
        }

        max
    }
}

struct Solution {}
