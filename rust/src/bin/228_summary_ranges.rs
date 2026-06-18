impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let Some(mut start) = nums.first().copied() else {
            return Vec::new();
        };
        let mut result = Vec::new();
        let mut prev = start;

        for &n in nums.iter().skip(1) {
            if n == prev {
                continue;
            }

            if prev + 1 == n {
                prev = n;
                continue;
            }

            let mut range = start.to_string();
            if start != prev {
                range.push_str("->");
                range.push_str(&prev.to_string());
            }
            prev = n;
            start = n;
            result.push(range);
        }
        let mut range = start.to_string();
        if start != prev {
            range.push_str("->");
            range.push_str(&prev.to_string());
        }
        result.push(range);
        result
    }
}

struct Solution {}
