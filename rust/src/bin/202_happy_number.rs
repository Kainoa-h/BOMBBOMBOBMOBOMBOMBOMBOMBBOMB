// all unhappy loops in base 10 end up at 4
impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        while n != 1 && n != 4 {
            let mut acc = 0;
            while n > 0 {
                acc += (n % 10).pow(2);
                n /= 10;
            }
            n = acc;
        }
        n == 1
    }
}

struct Solution {}
