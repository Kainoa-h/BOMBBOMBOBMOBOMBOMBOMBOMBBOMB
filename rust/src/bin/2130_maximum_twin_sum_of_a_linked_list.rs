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
        let mut vec = Vec::new();
        let mut h = &head;
        while let Some(node) = &h {
            h = &node.next;
            vec.push(node.val);
        }
        let mut max = 0;
        for i in 0..vec.len() / 2 {
            max = max.max(vec[i] + vec[vec.len() - 1 - i]);
        }
        max
    }
}

struct Solution {}
