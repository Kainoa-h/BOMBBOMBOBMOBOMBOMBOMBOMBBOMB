impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sorted = nums
            .iter()
            .cloned()
            .enumerate()
            .collect::<Vec<(usize, i32)>>();
        sorted.sort_by_cached_key(|&x| {
            (
                std::iter::successors(Some(x.1), |&n| {
                    let next = n / 10;
                    (next > 0).then_some(next)
                })
                .map(|n| n % 10)
                .sum::<i32>(),
                x.1,
            )
        });

        let mut og_index_map = vec![0; n];
        for (idx, &(og_idx, _)) in sorted.iter().enumerate() {
            og_index_map[idx] = og_idx;
        }

        let mut visited = vec![false; n];
        let mut cycles = 0;
        for i in 0..n {
            if !visited[i] {
                cycles += 1;

                let mut curr = i;
                while !visited[curr] {
                    visited[curr] = true;
                    curr = og_index_map[curr];
                }
            }
        }

        (n - cycles) as i32
    }
}

struct Solution {}
