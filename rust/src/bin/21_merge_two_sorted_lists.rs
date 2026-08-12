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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut root = None;
        let mut tail = &mut root;

        while let (Some(l1),  Some(l2)) = (&list1, &list2) {
            if l1.val < l2.val {
                *tail = list1;
                tail = &mut tail.as_mut().unwrap().next;
                list1 = tail.take();
            } else {
                *tail = list2;
                tail = &mut tail.as_mut().unwrap().next;
                list2 = tail.take();
            }
        }

        *tail = list1.or(list2);

        root
    }
}

struct Solution {}
