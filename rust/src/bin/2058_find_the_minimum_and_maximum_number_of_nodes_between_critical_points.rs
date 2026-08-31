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
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let Some(head) = head else {
            return vec![-1, -1];
        };
        let mut prev_val = head.val;
        let mut curr = head.next.as_deref();
        let mut index = 1;

        let mut first_critical = None::<i32>;
        let mut recent_critical = None::<i32>;
        let (mut min_dist, mut max_dist) = (i32::MAX, 0);
        while let Some(node) = curr {
            let Some(next_node) = node.next.as_deref() else {
                break;
            };

            if (node.val < next_node.val && node.val < prev_val)
                || (node.val > next_node.val && node.val > prev_val)
            {
                if let Some(critical) = first_critical {
                    max_dist = max_dist.max(index - critical);
                } else {
                    first_critical = Some(index);
                }
                if let Some(critical) = recent_critical {
                    min_dist = min_dist.min(index - critical);
                }
                recent_critical = Some(index);
            }

            prev_val = node.val;
            curr = node.next.as_deref();
            index += 1;
        }

        if min_dist == i32::MAX {
            vec![-1, -1]
        } else {
            vec![min_dist, max_dist]
        }
    }
}

struct Solution {}
