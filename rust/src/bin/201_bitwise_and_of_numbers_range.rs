impl Solution {
    pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
        let lz = left.leading_zeros();
        let rz = right.leading_zeros();
        if lz != rz {
            return 0;
        }
        let mut shift = 0;
        while right > left {
            right >>= 1;
            left >>= 1;
            shift += 1;
        }
        right << shift
    }
}

struct Solution {}

fn main() {
    assert_eq!(4, Solution::range_bitwise_and(5, 7));
}
