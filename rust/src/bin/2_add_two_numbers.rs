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
        let mut head = ListNode {
            val: 0, next: None
        };
        let mut node = &mut head;
        while l1.is_some() || l2.is_some() {
            let n1 = l1.as_ref().map_or(0, |x|x.val);
            let n2 = l2.as_ref().map_or(0, |x|x.val);
            let sum = n1 + n2 + carry;
            let digit = sum % 10;

            node.next = Some(Box::new(ListNode{ val: digit, next: None}));
            node = node.next.as_mut().unwrap();

            carry = sum / 10;
            l1 = l1.map_or(None, |x| x.next);
            l2 = l2.map_or(None, |x| x.next);
        }
        if carry != 0 {
            node.next = Some(Box::new(ListNode{ val: carry, next: None}));
        }
        head.next
    }
}

struct Solution {}
