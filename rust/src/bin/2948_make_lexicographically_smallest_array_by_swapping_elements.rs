use std::collections::HashMap;

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut dsu = (0..nums.len()).collect::<Vec<usize>>();
        let mut sorted = nums
            .iter()
            .enumerate()
            .map(|x| (x.0, *x.1))
            .collect::<Vec<(usize, i32)>>();
        sorted.sort_unstable_by_key(|x| x.1);
        fn find(dsu: &mut Vec<usize>, i: usize) -> usize {
            if dsu[i] == i {
                return i;
            }
            dsu[i] = find(dsu, dsu[i]);
            dsu[i]
        }

        for w in sorted.windows(2) {
            let (a, b) = (w[0], w[1]);
            if b.1 - a.1 <= limit {
                let b_root = find(&mut dsu, b.0);
                let a_root = find(&mut dsu, a.0);
                dsu[b_root] = a_root;
            }
        }

        let mut map = HashMap::<usize,Vec<i32>>::new();
        for (idx, &val) in nums.iter().enumerate() {
            let root = find(&mut dsu, idx);
            map.entry(root).or_default().push(val);
        }

        for l in map.values_mut() {
            l.sort_unstable_by_key(|x|-x);
        }

        (0..nums.len()).map(|idx| {
            let root = find(&mut dsu, idx);
            map.get_mut(&root).unwrap().pop().unwrap()
        }).collect()
    }
}

struct Solution {}
