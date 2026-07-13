impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut result = Vec::new();

        let (low_len, high_len) = (low.ilog10() as i32 + 1, high.ilog10() as i32 + 1);

        fn most_significant_digit_log(n: i32) -> i32 {
            if n == 0 {
                return 0;
            }
            let log = (n as f64).log10().floor() as u32;
            n / 10_i32.pow(log)
        }

        let mut len = low_len;
        let mut start = most_significant_digit_log(low);
        loop {
            if start + len > 10 {
                start = 1;
                len += 1;
            }

            if len > 9 {
                break;
            }

            let candidate = (start..start + len)
                .rev()
                .enumerate()
                .fold(0, |acc, (idx, x)| acc + (x * 10_i32.pow(idx as u32)));

            if candidate < low {
                start += 1;
                continue;
            }

            if len < high_len || candidate <= high {
                result.push(candidate);
                start += 1;
            } else {
                break;
            }
        }
        result
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::sequential_digits(100, 300), vec![123, 234]);
}
