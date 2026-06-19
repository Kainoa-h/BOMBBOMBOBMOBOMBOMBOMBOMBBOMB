impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let Some(mut start) = nums.first().copied() else {
            return Vec::new();
        };
        let mut result = Vec::new();
        let mut prev = start;

        for &n in nums.iter().skip(1) {
            if n == prev || prev + 1 == n {
                prev = n;
                continue;
            }

            result.push(if start != prev {
                format!("{}->{}", start, prev)
            } else {
                start.to_string()
            });

            prev = n;
            start = n;
        }

        result.push(if start != prev {
            format!("{}->{}", start, prev)
        } else {
            start.to_string()
        });

        result
    }
}

struct Solution {}
