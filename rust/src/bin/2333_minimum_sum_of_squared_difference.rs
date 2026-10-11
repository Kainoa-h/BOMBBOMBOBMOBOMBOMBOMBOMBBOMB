impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let mut moves = (k1 + k2) as i64;
        let mut diff_sum = 0;
        let mut diffs = nums1
            .iter()
            .zip(nums2.iter())
            .map(|(&a, &b)| {
                let x = (a - b).abs() as i64;
                diff_sum += x;
                x
            })
            .collect::<Vec<_>>();
        let n = diffs.len();
        if diff_sum < moves {
            return 0;
        }
        if diffs.len() == 1 {
            return (diffs[0] - moves).max(0).pow(2);
        }
        diffs.sort_by_key(|k| -k);

        let mut plateau_len = 1;
        while plateau_len < n {
            let diff = diffs[plateau_len - 1] - diffs[plateau_len];
            let needed = diff * plateau_len as i64;

            if moves >= needed {
                moves -= needed;
                plateau_len += 1;
            } else {
                break;
            }
        }

        let max_height = diffs[plateau_len - 1];
        let div = plateau_len as i64;
        let partial_subs = moves / div;
        let remainder = (moves % div) as usize;

        let max_height = max_height - partial_subs;

        for x in diffs.iter_mut().take(plateau_len) {
            *x = max_height;
        }

        for x in diffs.iter_mut().take(remainder) {
            *x -= 1;
        }

        diffs.into_iter().map(|x| x.pow(2)).sum()
    }
}

struct Solution {}
