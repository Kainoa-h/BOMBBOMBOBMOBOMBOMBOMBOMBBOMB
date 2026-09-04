impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let lz = left.leading_zeros();
        let rz = right.leading_zeros();
        if lz != rz {
            return 0;
        }

        let inter = left & right;
        let mut result = 0;
        for i in (0..(32 - lz)).rev() {
            if (inter >> i) & 1 != 1 {
                break;
            }
            result |= 1 << i;
        }
        result
    }
}

struct Solution {}

fn main() {
    assert_eq!(4, Solution::range_bitwise_and(5, 7));
}
