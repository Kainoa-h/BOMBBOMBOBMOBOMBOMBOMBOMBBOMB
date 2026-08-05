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
            let n1 = l1.as_ref().map_or(0, |x| x.val);
            let n2 = l2.as_ref().map_or(0, |x| x.val);
            let sum = n1 + n2 + carry;
            carry = sum / 10;

            l1 = l1.and_then(|x| x.next);
            l2 = l2.and_then(|x| x.next);

            node.next = Some(Box::new(ListNode::new(sum % 10)));
            node = node.next.as_mut().unwrap();
        }
        head.next
    }
}

struct Solution {}
