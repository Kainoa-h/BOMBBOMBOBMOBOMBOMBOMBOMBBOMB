impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        if n % 10 == 0 {
            return n;
        }

        let incr = |oneth, tenth| {
            let no = oneth + 1;
            if no == 10 {
                return (0, tenth + 1);
            }
            (no, tenth)
        };

        let (mut oneth, mut tenth) = (n % 10, (n / 10) % 10);

        let mut prod = oneth * tenth.max(1);
        while prod % t != 0 {
            (oneth, tenth) = incr(oneth, tenth);
            prod = oneth * tenth.max(1);
        }

        tenth * 10 + oneth
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::smallest_number(1, 2), 2);
}
