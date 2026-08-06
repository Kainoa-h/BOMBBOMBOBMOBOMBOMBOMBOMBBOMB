impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        if n % 10 == 0 {
            return n;
        }

        let (mut ones, mut tens) = (n % 10, (n / 10) % 10);

        while (ones * tens.max(1)) % t != 0 {
            ones += 1;
            if ones == 10 {
                ones = 0;
                tens += 1;
            }
        }

        tens * 10 + ones
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::smallest_number(1, 2), 2);
}
