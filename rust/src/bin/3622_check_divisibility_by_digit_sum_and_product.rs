impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let mut x = n;
        let mut sum = 0;
        let mut prod = 1;
        while x > 0 {
            let d = x % 10;
            x /= 10;
            sum += d;
            prod *= d;
        }
        n % (sum + prod) == 0
    }
}

struct Solution {}
