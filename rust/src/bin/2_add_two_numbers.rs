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
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut head = ListNode::new(0);
        let mut node = &mut head;
        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;

            if let Some(n) = l1 {
                sum += n.val;
                l1 = n.next;
            }

            if let Some(n) = l2 {
                sum += n.val;
                l2 = n.next;
            }

            carry = sum / 10;

            node.next = Some(Box::new(ListNode::new(sum % 10)));
            node = node.next.as_mut().unwrap();
        }
        head.next
    }
}

struct Solution {}
