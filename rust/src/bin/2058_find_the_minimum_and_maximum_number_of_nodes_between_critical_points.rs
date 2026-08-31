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
        let mut index = 1;
        let mut first_critical = None::<i32>;
        let mut recent_critical = None::<i32>;
        let (mut min_dist, mut max_dist) = (None::<i32>, None::<i32>);
        let Some(root) = head else {
            return vec![-1, -1];
        };
        let mut prev_val = root.val;
        let mut next = root.next.as_ref();
        while let Some(curr) = next {
            let val = curr.val;
            let Some(next_node) = curr.next.as_ref() else {
                break;
            };

            if (val < next_node.val && val < prev_val) || (val > next_node.val && val > prev_val){
                if first_critical.is_none() {
                    first_critical = Some(index);
                    continue;
                }
                if let Some(critical) = recent_critical {
                    min_dist = Some(min_dist.unwrap_or(i32::MAX).min(index - critical));
                }
                if let Some(critical) = first_critical {
                    max_dist = Some(max_dist.unwrap_or_default().max(index - critical));
                }
                recent_critical = Some(index);
            }

            prev_val = val;
            next = curr.next.as_ref();
            index += 1;
        }

        match (min_dist, max_dist) {
            (Some(min), Some(max)) => vec![min, max],
            _ => vec![-1, -1],
        }
    }
}

struct Solution {}
