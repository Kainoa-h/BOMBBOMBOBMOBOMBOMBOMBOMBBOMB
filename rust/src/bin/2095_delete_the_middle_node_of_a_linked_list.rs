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
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut count_head = head.as_ref();
        let mut middle_index = 0;
        while let Some(fast_node) = count_head {
            count_head = fast_node.next.as_ref();
            middle_index += 1;
        }
        middle_index /= 2;

        if middle_index == 0 {
            return None;
        }

        let mut bef_middle_head = head.as_mut();
        for _ in 0..middle_index - 1 {
            bef_middle_head = bef_middle_head.unwrap().next.as_mut();
        }

        if let Some(bef_mid_node) = bef_middle_head {
            let middle_head = bef_mid_node.next.take();
            let mut mid_node = middle_head?;
            bef_mid_node.next = mid_node.next.take();
        }

        head
    }
}

struct Solution {}
