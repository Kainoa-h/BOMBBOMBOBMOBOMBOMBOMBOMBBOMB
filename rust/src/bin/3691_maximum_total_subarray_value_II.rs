use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::Hash;

#[derive(Debug, Clone, Copy)]
struct SubArray {
    val: i32,
    start_incl: usize,
    end_incl: usize,
    largest: i32,
    smallest: i32,
}

impl SubArray {
    fn new(nums: &[i32], start_incl: usize, end_incl: usize) -> Self {
        let n = &nums[start_incl..=end_incl];
        let largest = *n.iter().max().unwrap();
        let smallest = *n.iter().min().unwrap();

        SubArray {
            val: largest - smallest,
            start_incl,
            end_incl,
            largest,
            smallest,
        }
    }

    fn derive_new(
        &self,
        nums: &[i32],
        new_start: usize,
        new_end: usize,
        remove_index: usize,
    ) -> Self {
        if new_end < new_start {
            return SubArray {
                val: -1,
                start_incl: 0,
                end_incl: 0,
                largest: -1,
                smallest: -1,
            };
        }
        if nums[remove_index] != self.largest && nums[remove_index] != self.smallest {
            let mut x = *self;
            x.start_incl = new_start;
            x.end_incl = new_end;
            return x;
        }

        let mut largest = self.largest;
        let mut smallest = self.smallest;

        if nums[remove_index] == self.largest {
            largest = *nums[new_start..=new_end].iter().max().unwrap();
        } else {
            smallest = *nums[new_start..=new_end].iter().min().unwrap();
        }

        SubArray {
            val: largest - smallest,
            start_incl: new_start,
            end_incl: new_end,
            largest,
            smallest,
        }
    }
}

impl PartialEq for SubArray {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
            && self.start_incl == other.start_incl
            && self.end_incl == other.end_incl
    }
}

impl Hash for SubArray {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.val.hash(state);
        self.start_incl.hash(state);
        self.end_incl.hash(state);
    }
}

impl Eq for SubArray {}

impl Ord for SubArray {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_diff = self.end_incl.saturating_sub(self.start_incl);
        let other_diff = other.end_incl.saturating_sub(other.start_incl);

        (self.val, self_diff, self.start_incl, self.end_incl).cmp(&(
            other.val,
            other_diff,
            other.start_incl,
            other.end_incl,
        ))
    }
}

impl PartialOrd for SubArray {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, mut k: i32) -> i64 {
        let mut max_sum = 0;
        let mut heap = BinaryHeap::new();
        let seed = SubArray::new(&nums, 0, nums.len() - 1);
        heap.push(seed);

        while k != 0 {
            let large = heap.pop().unwrap();
            max_sum += large.val as i64;

            let right = large.derive_new(
                &nums,
                large.start_incl + 1,
                large.end_incl,
                large.start_incl,
            );
            heap.push(right);

            if large.start_incl == 0 {
                let left = large.derive_new(
                    &nums,
                    large.start_incl,
                    large.end_incl.saturating_sub(1),
                    large.end_incl,
                );
                heap.push(left);
            }
            k -= 1;
        }
        max_sum
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::max_total_value(vec![1, 3, 2], 2), 4);
    assert_eq!(Solution::max_total_value(vec![4, 2, 5, 1], 3), 12);
    assert_eq!(Solution::max_total_value(vec![22], 1), 0);
}
