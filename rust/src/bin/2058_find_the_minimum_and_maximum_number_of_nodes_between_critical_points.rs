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
        let Some(mut head) = head else {
            return vec![-1, -1];
        };
        let mut prev_val = head.val;
        let mut curr = head.next.take();
        let mut index = 1;

        let mut first_critical = None::<i32>;
        let mut recent_critical = None::<i32>;
        let mut min_dist = i32::MAX;
        while let Some(mut node) = curr {
            let Some(next_node) = node.next.take() else {
                break;
            };

            if (node.val < next_node.val && node.val < prev_val)
                || (node.val > next_node.val && node.val > prev_val)
            {
                if let Some(critical) = recent_critical {
                    min_dist = min_dist.min(index - critical);
                } else {
                    first_critical = Some(index);
                }
                recent_critical = Some(index);
            }

            prev_val = node.val;
            curr = Some(next_node);
            index += 1;
        }

        match (first_critical, recent_critical) {
            (Some(f), Some(l)) if f != l => vec![min_dist, l-f],
            _=> vec![-1, -1]
            
        }
    }
}

struct Solution {}
